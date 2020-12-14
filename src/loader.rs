use detour::Error as DetourError;
use detour::RawDetour;
use log::LevelFilter;
use simplelog::{CombinedLogger, ConfigBuilder, WriteLogger};
use std::{ffi::CString, fs::File, iter};
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress};

use crate::consts::UPLAY_R1_ORIGINAL_DLL_NAME;
use crate::exports::uplay_arch::uplay_ach_earn_achievement;

use crate::{
    exports::uplay_arch::*, exports::uplay_avatar::*, exports::uplay_common::*,
    exports::uplay_friends::*, exports::uplay_installer::*, exports::uplay_metadata::*,
    exports::uplay_options::*, exports::uplay_overlay::*, exports::uplay_party::*,
    exports::uplay_presence::*, exports::uplay_save::*, exports::uplay_user::*,
    exports::uplay_win::*,
};

struct UplayPtr<T>(pub *const T);

unsafe impl<T> Send for UplayPtr<T> {}
unsafe impl<T> Sync for UplayPtr<T> {}

static HOOKS: &'static [(&str, UplayPtr<()>)] = include!(concat!(env!("OUT_DIR"), "/hooks.rs"));

#[inline]
pub fn init_logger(path: &str) {
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_target_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Debug)
            .set_time_format_str("%F %T%.3f")
            .build(),
        File::create(path).unwrap(),
    )])
    .unwrap();
}

#[inline]
pub unsafe fn init_hooks() {
    let get_module_symbol_address = |module: &str, symbol: &str| -> Option<usize> {
        let module = module
            .encode_utf16()
            .chain(iter::once(0))
            .collect::<Vec<u16>>();
        let symbol = CString::new(symbol).unwrap();
        let handle = GetModuleHandleW(module.as_ptr());

        match GetProcAddress(handle, symbol.as_ptr()) as usize {
            0 => None,
            n => Some(n),
        }
    };

    for (name, address) in HOOKS {
        if let Some(org_address) = get_module_symbol_address(UPLAY_R1_ORIGINAL_DLL_NAME, name) {
            info!("Try to install hook: {}", &name);

            let org_address = org_address as *const ();
            let hook_address = address.0;

            if let Err(err) = || -> Result<(), DetourError> {
                let detour = Box::leak(Box::new(RawDetour::new(org_address, hook_address)?));
                detour.enable()?;
                Ok(())
            }() {
                error!("{}", err)
            }
        } else {
            warn!("Function not found: {}", name);
        }
    }
}
