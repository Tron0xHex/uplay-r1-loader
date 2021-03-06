use crate::{consts::SAVES_MANIFEST_NAME, models::manifest::Manifest};
use std::{fs, io, io::Error as IoError, path::PathBuf};
use thiserror::Error;
use toml::ser::Error as TomlError;

use super::save::get_saves_path;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Toml error: {0:?}")]
    Toml(#[from] TomlError),
    #[error("Io error: {0:?}")]
    Io(#[from] IoError),
}

#[inline]
pub fn get_manifest_path() -> PathBuf {
    let mut saves_path = get_saves_path();

    saves_path.push(SAVES_MANIFEST_NAME);
    saves_path
}

#[inline]
pub fn read_manifest() -> io::Result<Manifest> {
    let manifest_path = get_manifest_path();

    let manifest_str = fs::read_to_string(manifest_path)?;
    let manifest = toml::from_str(&manifest_str)?;

    Ok(manifest)
}

#[inline]
pub fn write_manifest(manifest: &Manifest) -> Result<(), Error> {
    let saves_path = get_saves_path();

    if !saves_path.exists() {
        fs::create_dir_all(saves_path)?;
    }

    let manifest_path = get_manifest_path();
    let manifest_str = toml::to_string(manifest)?;

    fs::write(manifest_path, manifest_str)?;

    Ok(())
}
