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

pub fn backup_asset(library: &str, backup_directory: &str, asset: &Asset, simulate: bool) -> Result<(), io::Error> {
    let backup_directory = &format!("{}/{}", &backup_directory, asset.album_name);
    check_path_exists_or_create(backup_directory)?;
    let source = format!("{}/originals/{}/{}", library, asset.dir, asset.filename);
    let target = format!("{}/{}", backup_directory, asset.original_filename);
    if simulate {
        println!("cp {} {}", &source, &target);
    } else {
        fs::copy(&source, &target)?;
    }
    Ok(())
}