use std::{fmt::Debug, os::raw::c_char};

#[repr(C)]
#[derive(Debug)]
pub struct UplaySave {
    pub slot_id: u32,
    pub name: *const c_char,
}

impl UplaySave {
    #[inline]
    pub fn new(slot_id: u32, name: *const c_char) -> Self {
        Self { slot_id, name }
    }
}
