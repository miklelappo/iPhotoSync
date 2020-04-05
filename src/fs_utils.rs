use crate::db;

use std::path::Path;
use std::fs;
use std::io;
use db::Asset;
use indicatif::{ ProgressBar, ProgressStyle };

fn check_path_exists_or_create(path: &str, simulate: bool) -> Result<(), io::Error> {
    let path = Path::new(path);
    if path.exists() {
        Ok(())
    } else {
        if simulate {
            println!("Create {}", path.to_str().unwrap());
        } else {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }
}

fn backup_asset(library: &str, backup_directory: &str, asset: &Asset, simulate: bool) -> Result<(), io::Error> {
    let backup_directory = &format!("{}/{}", &backup_directory, asset.album_name);
    check_path_exists_or_create(backup_directory, simulate)?;
    let source = format!("{}/originals/{}/{}", library, asset.dir, asset.filename);
    let target = format!("{}/{}", backup_directory, asset.original_filename);
    if simulate {
        println!("cp {} {}", &source, &target);
    } else {
        fs::copy(&source, &target)?;
    }
    Ok(())
}

pub fn backup_assets(library: &str, backup_directory: &str, assets: &Vec<Asset>, simulate: bool) -> Result<(), io::Error> {
    check_path_exists_or_create(&backup_directory, simulate)?;
    let progress_bar = ProgressBar::new(assets.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .progress_chars("##-"));

    for asset in assets.iter() {
        progress_bar.inc(1);
        backup_asset(&library, &backup_directory, &asset, simulate)?;
    }
    progress_bar.finish_with_message("done");
    Ok(())
}