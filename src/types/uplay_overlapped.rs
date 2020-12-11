use std::{ffi::c_void, ptr::null_mut};

#[repr(C)]
#[derive(Debug)]
pub struct UplayOverlapped {
    pub result: *const c_void,
    pub is_completed: u32,
    pub reserved: i32,
}

impl UplayOverlapped {
    pub fn set_zeros(&mut self) {
        self.result = null_mut();
        self.is_completed = 0;
        self.reserved = 0;
    }

    pub fn set_result(&mut self, result: Option<*const c_void>) {
        match result {
            Some(result) => {
                self.result = result;
                self.is_completed = 1;
            }
            None => unsafe {
                self.result = self.result.add(1);
                self.is_completed = 1;
                self.reserved = 0;
            },
        }
    }
}
