pub mod consts;
pub mod exports;
pub mod global;
pub mod helpers;
pub mod loader;
pub mod models;
pub mod types;

#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use global::CONFIG;
use loader::{init_hooks, init_logger};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};

#[export_name = "SpaceCat"]
pub fn space_cat() {}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "system" fn DllMain(
    dll_module: HINSTANCE,
    call_reason: DWORD,
    reserved: LPVOID,
) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;
    const DLL_PROCESS_DETACH: DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => {
            if CONFIG.uplay.log.write {
                init_logger(&CONFIG.uplay.log.path);
            }

            if CONFIG.uplay.install_hooks {
                init_hooks();
            }
        }
        DLL_PROCESS_DETACH => {}
        _ => (),
    }
    TRUE
}
