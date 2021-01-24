use std::ffi::c_void;

#[export_name = "UPLAY_PRESENCE_SetPresence"]
pub fn uplay_presence_set_presence(_presence_id: u32, _tokens: *const c_void) -> usize {
    debug!("UPLAY_PRESENCE_SetPresence");
    return 1;
}
