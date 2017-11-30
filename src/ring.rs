// rte_ring.h
use ffi;
use std::ffi::CString;
use std::os::raw::c_void;

pub unsafe fn create(
    name: &'static str,
    count: u32,
    socket_id: i32,
    flags: u32,
) -> *mut ffi::rte_ring {
    ffi::rte_ring_create(
        CString::new(name).unwrap().into_raw(),
        count,
        socket_id,
        flags,
    )
}

impl ffi::rte_ring {
    pub unsafe fn free(&mut self) {
        ffi::rte_ring_free(self);
    }

    pub unsafe fn do_enqueue_sp(&mut self, objs: *const *mut c_void, n: u32) -> u32 {
        let spc = self.capacity + self.cons.tail - self.prod.head;
        let mut copy_size = n;
        if copy_size > spc {
            copy_size = spc;
        }
        self.prod.head += copy_size;
        let idx = self.prod.head & self.mask;
        let ring = (self as *mut ffi::rte_ring).offset(1) as *mut *mut c_void;
        if idx + copy_size < self.size {
            for i in 0..copy_size as isize {
                *ring.offset(idx as isize + i) = *objs.offset(i);
            }
        } else {
            let hsz = (self.size - idx) as isize;
            for i in 0..hsz {
                *ring.offset(idx as isize + i) = *objs.offset(i);
            }
            for j in 0..copy_size as isize - hsz {
                *ring.offset(j) = *objs.offset(hsz + j);
            }
        }
        self.prod.tail = self.prod.head;
        n
    }

    pub unsafe fn do_dequeue_sc(&mut self, objs: *mut *mut c_void, n: u32) {
        let spc = self.prod.tail - self.cons.head;
        let mut copy_size = n;
        if copy_size > spc {
            copy_size = spc;
        }
        self.cons.head += copy_size;
        let idx = self.cons.head & self.mask;
        let ring = (self as *mut ffi::rte_ring).offset(1) as *mut *mut c_void;
        if idx + copy_size < self.size {
            for i in 0..copy_size as isize {
                *objs.offset(i) = *ring.offset(idx as isize + i);
            }
        } else {
            let hsz = (self.size - idx) as isize;
            for i in 0..hsz {
                *objs.offset(i) = *ring.offset(idx as isize + i);
            }
            for j in 0..copy_size as isize - hsz {
                *objs.offset(hsz + j) = *ring.offset(j);
            }
        }
        self.cons.tail = self.cons.head;
    }
}
