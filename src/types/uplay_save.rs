use std::{fmt::Debug, os::raw::c_char};

#[repr(C)]
#[derive(Debug)]
pub struct UplaySave {
    pub slot_id: u32,
    #[cfg(feature = "next-gen-api")]
    pad: u32,
    pub name: *const c_char,
}

impl UplaySave {
    #[inline]
    pub fn new(slot_id: u32, name: *const c_char) -> Self {
        #[cfg(feature = "next-gen-api")]
        {
            Self {
                slot_id,
                pad: 0,
                name,
            }
        }

        #[cfg(not(feature = "next-gen-api"))]
        {
            Self { slot_id, name }
        }
    }
}
