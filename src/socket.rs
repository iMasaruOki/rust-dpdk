use ffi;

pub unsafe fn id() -> i32 {
    ffi::rte_socket_id() as i32
}
