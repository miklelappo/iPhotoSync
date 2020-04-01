use crate::db;

use std::path::Path;
use std::fs;
use std::io;
use db::Asset;

pub fn check_path_exists_or_create(path: &str) -> Result<(), io::Error> {
    let path = Path::new(path);
    if path.exists() {
        Ok(())
    } else {
        println!("Create {}", path.to_str().unwrap());
        fs::create_dir_all(path)?;
        Ok(())
    }
}

pub fn backup_asset(library: &str, backup_directory: &str, asset: &Asset) -> Result<(), io::Error> {
    let backup_directory = &format!("{}/{}", &backup_directory, asset.album_name);
    check_path_exists_or_create(backup_directory)?;
    fs::copy(
        &format!("{}/originals/{}/{}", library, asset.dir, asset.filename),
        &format!("{}/{}", backup_directory, asset.original_filename)
    )?;
    Ok(())
}