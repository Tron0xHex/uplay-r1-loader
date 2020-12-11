use std::{fmt::Debug, os::raw::c_char};

#[repr(C)]
#[derive(Debug)]
pub struct UplayKey {
    pub cd_key: *const c_char,
}
