use crate::infra::wasm::{copy_to_vm, to_host_ptr};
use crate::op;
use crate::profiling::runtime::Data;
use profiling_prelude_perf_types::config::{Cpu, Process};
use profiling_prelude_perf_types::counting::{Config, CounterStat};
use profiling_prelude_perf_types::{raw_parts_de, ser};
use wasmtime::Caller;

pub fn new_counter(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    sered_process_vm_ptr: u32,
    sered_process_len: u32,
    sered_cpu_vm_ptr: u32,
    sered_cpu_len: u32,
    sered_cfg_vm_ptr: u32,
    sered_cfg_len: u32,
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

    let cfg: Config = unsafe {
        let ptr = to_host_ptr(caller, sered_cfg_vm_ptr);
        raw_parts_de(ptr as _, sered_cfg_len as _)
    };

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match op::raw::perf::counting::new_counter(&cfg).map(|it| caller.data_mut().add_resource(it)) {
        Ok(counter_rid) => {
            ret_area[0] = 1;
            ret_area[1] = counter_rid;
        }
        Err(e) => {
            let e = e.to_string();
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn enable_counter(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;
    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid")
        .map(op::raw::perf::counting::enable_counter);

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn disable_counter(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;
    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid")
        .map(op::raw::perf::counting::disable_counter);

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn reset_counter_count(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;
    let result = caller
        .data()
        .get_resource(counter_rid)
        .ok_or("Invalid rid")
        .map(op::raw::perf::counting::reset_counter_count);

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(_) => {
            ret_area[0] = 1;
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}

pub fn get_counter_stat(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;
    let stat = caller
        .data_mut()
        .get_resource_mut(counter_rid)
        .ok_or_else(|| "Invalid rid".to_string())
        .and_then(|it| op::raw::perf::counting::get_counter_stat(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match stat {
        Ok(stat) => {
            let result = CounterStat {
                event_id: stat.event_id,
                event_count:  stat.event_count,
                time_enabled: stat.time_enabled,
                time_running: stat.time_running,
            };
            let sered_cr = ser(&result);
            let vm_ptr = unsafe { copy_to_vm(caller, sered_cr.as_ref()) };

            *ret_area = [1, vm_ptr, sered_cr.len() as _];
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}
