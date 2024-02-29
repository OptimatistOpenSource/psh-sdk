mod raw;

use crate::convert::Wrap;
use crate::profiling::perf::counter_group::*;
use crate::{FixedCounterGroup, PerfView};
use perf_event_rs::counting::{CounterGroup, CounterGuard};
use wasmtime::component::Resource;

impl<T> HostCounterGroup for T
where
    T: PerfView,
{
    fn new(
        &mut self,
        process: Process,
        cpu: Cpu,
    ) -> wasmtime::Result<Result<Resource<CounterGroup>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group = raw::counter_group_new(&process, &cpu)?;
            let handle = PerfView::table_mut(self).push(counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn add_member(
        &mut self,
        self_: Resource<CounterGroup>,
        cfg: Config,
    ) -> wasmtime::Result<Result<Resource<CounterGuard>, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group: &mut CounterGroup = PerfView::table_mut(self).get_mut(&self_)?;
            let guard = raw::counter_group_add_member(counter_group, &cfg)?;
            let handle = PerfView::table_mut(self).push(guard)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn enable(
        &mut self,
        counter_group: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<Resource<FixedCounterGroup>, String>> {
        let f = || -> anyhow::Result<_> {
            let counter_group: CounterGroup = PerfView::table_mut(self).delete(counter_group)?;
            let fixed_counter_group = raw::counter_group_enable(counter_group)?;
            let handle = PerfView::table_mut(self).push(fixed_counter_group)?;
            Ok(handle)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(
        &mut self,
        self_: Resource<CounterGroup>,
    ) -> wasmtime::Result<Result<CounterGroupStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let counter_group: &mut CounterGroup = PerfView::table_mut(self).get_mut(&self_)?;
            let stat = raw::counter_group_stat(counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<CounterGroup>) -> wasmtime::Result<()> {
        PerfView::table_mut(self).delete(rep)?;
        Ok(())
    }
}

impl<T> HostFixedCounterGroup for T
where
    T: PerfView,
{
    fn enable(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
            raw::fixed_counter_group_enable(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn disable(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
            raw::fixed_counter_group_disable(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn reset(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<(), String>> {
        let f = || -> anyhow::Result<_> {
            let fixed_counter_group: &FixedCounterGroup = PerfView::table(self).get(&self_)?;
            raw::fixed_counter_group_reset(fixed_counter_group)?;
            Ok(())
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn stat(
        &mut self,
        self_: Resource<FixedCounterGroup>,
    ) -> wasmtime::Result<Result<CounterGroupStat, String>> {
        let mut f = || -> anyhow::Result<_> {
            let fixed_counter_group: &mut FixedCounterGroup =
                PerfView::table_mut(self).get_mut(&self_)?;
            let stat = raw::fixed_counter_group_stat(fixed_counter_group)?;
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
            Ok(stat)
        };
        Ok(f().map_err(|e| e.to_string()))
    }

    fn drop(&mut self, rep: Resource<FixedCounterGroup>) -> wasmtime::Result<()> {
        PerfView::table_mut(self).delete(rep)?;
        Ok(())
    }
}

pub fn counter_guard_stat(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_guard_rid: u32) {
    let caller = &mut caller;

    let stat = caller
        .data_mut()
        .get_resource_mut(counter_guard_rid)
        .ok_or_else(|| "Invalid rid".to_string())
        .and_then(|it| raw::counter_guard_stat(it).map_err(|e| e.to_string()));

    let mem = get_mem(caller);
    let ret_area_ptr = unsafe { to_host_ptr(mem, ret_area_vm_ptr) as *mut [u32; 3] };
    let ret_area = unsafe { &mut *ret_area_ptr };
    match stat {
        Ok(stat) => {
            let stat = Wrap::<CounterStat>::from(&stat).into_inner();
            let sered_stat = ser(&stat);
            let vm_ptr = unsafe { copy_to_vm(caller, sered_stat.as_ref()) };

            *ret_area = [1, vm_ptr, sered_stat.len() as _];
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}
