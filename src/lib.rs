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

use std::process::exit;

use global::CONFIG;
use loader::{init_hooks, init_logger};
use native_dialog::{Dialog, MessageAlert, MessageType};
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};

#[inline]
fn fail_if(condition: bool, msg: &str) {
    if condition {
        let dialog = MessageAlert {
            title: "Error...",
            text: &msg,
            typ: MessageType::Error,
        };
        dialog.show().unwrap();
        exit(1);
    }
}

#[inline]
fn check_config() {
    fail_if(CONFIG.uplay.saves.is_empty(), "Saves path is empty!");
    fail_if(CONFIG.uplay.profile.email.is_empty(), "Email is empty!");
    fail_if(
        CONFIG.uplay.profile.username.is_empty(),
        "Username is empty!",
    );
    fail_if(
        CONFIG.uplay.profile.password.is_empty(),
        "Password is empty!",
    );
    fail_if(
        CONFIG.uplay.profile.account_id.is_empty(),
        "Account id is empty!",
    );
    fail_if(
        CONFIG.uplay.log.write && CONFIG.uplay.log.path.is_empty(),
        "Log path is empty!",
    );
}

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
            check_config();

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
