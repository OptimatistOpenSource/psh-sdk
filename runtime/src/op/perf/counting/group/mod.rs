mod raw;
use crate::infra::wasm::{copy_to_vm, move_to_vm, to_host_ptr};
use crate::profiling::runtime::Data;
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::{Config, CounterGroupStat, CounterStat};
use profiling_prelude_perf_types::{raw_parts_de, ser};
use wasmtime::Caller;

use crate::op::perf::convert::Wrap;
use perf_event_rs::counting::CounterGroup as RawCounterGrp;

pub fn counter_group_new(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    sered_process_vm_ptr: u32,
    sered_process_len: u32,
    sered_cpu_vm_ptr: u32,
    sered_cpu_len: u32,
) {
    let caller = &mut caller;

    let process: Process = unsafe {
        let ptr = to_host_ptr(caller, sered_process_vm_ptr);
        raw_parts_de(ptr as _, sered_process_len as _)
    };

    let cpu: Cpu = unsafe {
        let ptr = to_host_ptr(caller, sered_cpu_vm_ptr);
        raw_parts_de(ptr as _, sered_cpu_len as _)
    };

    let counter_group_rid =
        raw::counter_group_new(&process, &cpu).map(|it| caller.data_mut().add_resource(it));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match counter_group_rid {
        Ok(counter_group_rid) => {
            ret_area[0] = 1;
            ret_area[1] = counter_group_rid;
        }
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_group_add_member(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    counter_group_rid: u32,
    sered_cfg_vm_ptr: u32,
    sered_cfg_len: u32,
) {
    let caller = &mut caller;

    let cfg: Config = unsafe {
        let ptr = to_host_ptr(caller, sered_cfg_vm_ptr);
        raw_parts_de(ptr as _, sered_cfg_len as _)
    };

    let counter_guard_rid = caller
        .data_mut()
        .get_resource_mut::<RawCounterGrp>(counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|counter_group| {
            raw::counter_group_add_member(counter_group, &cfg).map_err(|e| e.to_string())
        })
        .map(|it| caller.data_mut().add_resource(it));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match counter_guard_rid {
        Ok(counter_guard_rid) => {
            ret_area[0] = 1;
            ret_area[1] = counter_guard_rid;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_group_enable(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    counter_group_rid: u32,
) {
    let caller = &mut caller;

    let fixed_counter_guard_rid = caller
        .data_mut()
        .take_resource(counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_group_enable(*it).map_err(|e| e.to_string()))
        .map(|it| caller.data_mut().add_resource(it));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match fixed_counter_guard_rid {
        Ok(fixed_counter_guard_rid) => {
            ret_area[0] = 1;
            ret_area[1] = fixed_counter_guard_rid;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_group_stat(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_group_rid: u32) {
    let caller = &mut caller;

    let stat = caller
        .data_mut()
        .get_resource_mut(counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_group_stat(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match stat {
        Ok(stat) => {
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
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

pub fn fixed_counter_group_enable(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    fixed_counter_group_rid: u32,
) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(fixed_counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::fixed_counter_group_enable(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn fixed_counter_group_disable(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    fixed_counter_group_rid: u32,
) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(fixed_counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::fixed_counter_group_disable(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn fixed_counter_group_reset(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    fixed_counter_group_rid: u32,
) {
    let caller = &mut caller;

    let result = caller
        .data()
        .get_resource(fixed_counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::fixed_counter_group_reset(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn fixed_counter_group_stat(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    fixed_counter_group_rid: u32,
) {
    let caller = &mut caller;

    let stat = caller
        .data_mut()
        .get_resource_mut(fixed_counter_group_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::fixed_counter_group_stat(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match stat {
        Ok(stat) => {
            let stat = Wrap::<CounterGroupStat>::from(&stat).into_inner();
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

pub fn counter_guard_event_id(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    counter_guard_rid: u32,
) {
    let caller = &mut caller;

    let event_id = caller
        .data_mut()
        .get_resource_mut(counter_guard_rid)
        .ok_or("Invalid rid".to_string())
        .map(|it| raw::counter_guard_event_id(it));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match event_id {
        Ok(event_id) => {
            ret_area[0] = 1;
            ret_area[1] = unsafe { move_to_vm(caller, event_id) };
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn counter_guard_stat(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_guard_rid: u32) {
    let caller = &mut caller;

    let stat = caller
        .data_mut()
        .get_resource_mut(counter_guard_rid)
        .ok_or("Invalid rid".to_string())
        .and_then(|it| raw::counter_guard_stat(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
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
