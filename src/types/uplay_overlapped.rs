#[cfg(not(feature = "next-gen-api"))]
use std::ffi::c_void;
use std::ptr::null_mut;

#[repr(C)]
#[derive(Debug)]
pub struct UplayOverlapped {
    #[cfg(feature = "next-gen-api")]
    pub result: *const i32,
    #[cfg(not(feature = "next-gen-api"))]
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

    #[cfg(feature = "next-gen-api")]
    pub fn set_result(&mut self) {
        self.is_completed = 1;
        self.reserved = 0;
    }

    #[cfg(not(feature = "next-gen-api"))]
    pub fn set_result(&mut self, result: *const c_void) {
        self.result = result;
        self.is_completed = 1;
    }
}
