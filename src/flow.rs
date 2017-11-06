use ffi;

pub unsafe fn validate(port_id: u8,
                       attr: *const ffi::rte_flow_attr,
                       pattern: *const ffi::rte_flow_item,
                       actions: *const ffi::rte_flow_action,
                       error: *mut ffi::rte_flow_error) -> i32 {
    ffi::rte_flow_validate(port_id, attr, pattern, actions, error)
}

pub unsafe fn create(port_id: u8,
                     attr: *const ffi::rte_flow_attr,
                     pattern: *const ffi::rte_flow_item,
                     actions: *const ffi::rte_flow_action,
                     error: *mut ffi::rte_flow_error) -> *mut ffi::rte_flow {
    ffi::rte_flow_create(port_id, attr, pattern, actions, error)
}

pub unsafe fn destroy(port_id: u8,
                      flow: *mut ffi::rte_flow,
                      error: *mut ffi::rte_flow_error) -> i32 {
    ffi::rte_flow_destroy(port_id, flow, error)
}

pub unsafe fn flush(port_id: u8,
                    error: *mut ffi::rte_flow_error) -> i32 {
    ffi::rte_flow_flush(port_id, error)
}

pub unsafe fn query(port_id: u8,
                    flow: *mut ffi::rte_flow,
                    action: ffi::rte_flow_action_type,
                    data: *mut ::std::os::raw::c_void,
                    error: *mut ffi::rte_flow_error) -> i32 {
    ffi::rte_flow_query(port_id, flow, action, data, error)
}

pub unsafe fn isolate(port_id: u8,
                      set: bool,
                      error: *mut ffi::rte_flow_error) -> i32 {
    ffi::rte_flow_isolate(port_id, if set == true { 1 } else { 0 }, error)
}
