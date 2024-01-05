use crate::infra::wasm::{copy_to_vm, move_to_vm, to_host_ptr};
use crate::op;
use crate::op::raw::perf::CounterConfig;
use crate::profiling::runtime::Data;
use wasmtime::Caller;

pub fn new_counter(
    mut caller: Caller<Data>,
    ret_area_vm_ptr: u32,
    serialized_cfg_vm_ptr: u32,
    serialized_cfg_len: u32,
) {
    let caller = &mut caller;

    let serialized_cfg_ptr = unsafe { to_host_ptr(caller, serialized_cfg_vm_ptr) };

    // TODO: deserialize here...
    let _ = serialized_cfg_len;
    let cfg = unsafe { &*(serialized_cfg_ptr as *const CounterConfig) };

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match op::raw::perf::new_counter(cfg).map(|it| caller.data_mut().add_resource(it)) {
        Ok(id) => {
            ret_area[0] = 1;
            ret_area[1] = id;
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
        .map(op::raw::perf::enable_counter);

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
        .map(op::raw::perf::disable_counter);

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

#[repr(C)]
struct CounterResult {
    pub event_count: u64,
    pub time_enabled: u64,
    pub time_running: u64,
}

pub fn get_counter_result(mut caller: Caller<Data>, ret_area_vm_ptr: u32, counter_rid: u32) {
    let caller = &mut caller;
    let result = caller
        .data_mut()
        .get_resource_mut(counter_rid)
        .ok_or_else(|| "Invalid rid".to_string())
        .and_then(|it| op::raw::perf::get_counter_result(it).map_err(|e| e.to_string()));

    let ret_area = unsafe { &mut *(to_host_ptr(caller, ret_area_vm_ptr) as *mut [u32; 3]) };
    match result {
        Ok(result) => {
            // TODO: serialize here...
            let cr = CounterResult {
                event_count: result.event_count,
                time_enabled: result.time_enabled,
                time_running: result.time_running,
            };
            let vm_ptr = unsafe { move_to_vm(caller, cr) };
            let len = 0;

            *ret_area = [1, vm_ptr, len];
        }
        Err(e) => {
            let vm_ptr = unsafe { copy_to_vm(caller, e.as_str()) };
            *ret_area = [0, vm_ptr, e.len() as _];
        }
    }
}
