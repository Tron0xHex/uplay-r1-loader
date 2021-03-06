use fs::OpenOptions;
use std::{env, fs, io, io::prelude::*, io::Error as IoError, io::SeekFrom, path::PathBuf};
use thiserror::Error;

use crate::{
    consts::DEFAULT_SAVE_DATA_OFFSET, consts::SAVE_FILE_EXTENSION, global::CONFIG,
    models::manifest::Manifest,
};

use super::manifest::{get_manifest_path, read_manifest, write_manifest, Error as ManifestError};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid save: {0:?}")]
    InvalidSave(String),
    #[error("Manifest error: {0:?}")]
    Manifest(#[from] ManifestError),
    #[error("Io error: {0:?}")]
    Io(#[from] IoError),
}

#[inline]
pub fn get_saves_path() -> PathBuf {
    return match CONFIG.uplay.saves.as_str() {
        "<default>" => env::current_dir().unwrap().join("Saves"),
        "<roaming>" => dirs::config_dir()
            .unwrap()
            .join("UplayEmu")
            .join(&CONFIG.uplay.name)
            .join("Saves"),
        _ => PathBuf::from(&CONFIG.uplay.saves),
    };
}

#[inline]
pub fn get_save_path(id: u32) -> PathBuf {
    let mut path = get_saves_path();

    path.push(format!("{}.{}", id, SAVE_FILE_EXTENSION));
    path
}

#[inline]
pub fn get_saves() -> Result<Vec<(u32, String, PathBuf)>, Error> {
    let mut saves = Vec::new();

    let saves_path = get_saves_path();
    let manifest_path = get_manifest_path();

    if !saves_path.exists() || !manifest_path.exists() {
        return Ok(saves);
    }

    let manifest = read_manifest()?;

    for entry in fs::read_dir(&saves_path)? {
        let entry = entry?;
        let path = entry.path();
        let is_file = path.is_file();
        let extension = path.extension().unwrap().to_str().unwrap();

        if is_file && extension == SAVE_FILE_EXTENSION {
            let save_id = path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<i64>()
                .map_err(|_| Error::InvalidSave(entry.file_name().to_string_lossy().to_string()))?;

            if let Some(save) = manifest.saves.iter().find(|save| save.id == save_id) {
                saves.push((save.id as u32, save.name.clone(), path.clone()));
            }
        }
    }

    Ok(saves)
}

#[inline]
pub fn read_save(id: u32, num_of_bytes_to_read: u32, offset: u32) -> io::Result<(Vec<u8>, usize)> {
    let path = get_save_path(id);
    let mut file = OpenOptions::new().read(true).open(path)?;

    file.seek(SeekFrom::Start(
        DEFAULT_SAVE_DATA_OFFSET as u64 + offset as u64,
    ))?;

    let mut buffer = vec![0u8; num_of_bytes_to_read as usize];
    let read_bytes = file.read(&mut buffer)?;

    Ok((buffer, read_bytes))
}

#[inline]
pub fn write_save(
    id: u32,
    options: &OpenOptions,
    num_of_bytes_to_write: u32,
    buffer: &[u8],
) -> io::Result<()> {
    let path = get_saves_path();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let path = get_save_path(id);
    let mut file = options.open(path)?;

    file.seek(SeekFrom::Start(DEFAULT_SAVE_DATA_OFFSET as u64))?;
    file.write(&buffer[0..num_of_bytes_to_write as usize])?;

    Ok(())
}

#[inline]
pub fn remove_save(id: u32) -> Result<(), Error> {
    let path = get_save_path(id);
    let manifest = read_manifest()?;

    fs::remove_file(path)?;

    let saves = manifest
        .saves
        .iter()
        .cloned()
        .filter(|save| save.id != id as i64)
        .collect();

    write_manifest(&Manifest { saves })?;

    Ok(())
}
