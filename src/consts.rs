pub const CONFIG_NAME: &str = "Uplay.toml";
pub const SAVES_MANIFEST_NAME: &str = "Manifest.toml";
pub const SAVE_FILE_EXTENSION: &str = "save";
pub const DEFAULT_SAVE_DATA_OFFSET: usize = 520;

#[cfg(target_arch = "x86")]
pub const UPLAY_R1_ORIGINAL_DLL_NAME: &str = "uplay_r1_loader.dll";

#[cfg(target_arch = "x86_64")]
pub const UPLAY_R1_ORIGINAL_DLL_NAME: &str = "uplay_r1_loader64.dll";
