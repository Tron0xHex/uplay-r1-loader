#[repr(C)]
#[derive(Debug)]
pub struct UplayOverlapped {
    pub unk: u32,
    pub is_completed: u32,
    pub reserved: i32,
}

impl UplayOverlapped {
    #[inline]
    pub fn set_result(&mut self) {
        self.unk += 1;
        self.is_completed = 1;
        self.reserved = 0;
    }
}
