use crate::db;
use db::{ Asset };
use indicatif::{ProgressBar, ProgressStyle};
use std::{ fs, io };
use std::path::Path;

fn check_path_exists_or_create(path: &Path, dry_run: bool) -> Result<(), io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        if dry_run {
            println!("Create {}", path.display());
        } else {
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

fn backup_asset(
    library: &Path,
    backup_directory: &Path,
    asset: &Asset,
    simulate: bool,
) -> Result<(), io::Error> {
    let backup_directory = backup_directory.join(&asset.album_name);
    check_path_exists_or_create(backup_directory.as_path(), simulate)?;
    let source = library
        .join("originals")
        .join(&asset.dir)
        .join(&asset.filename);
    let target = backup_directory.join(&asset.original_filename);
    if simulate {
        println!("cp {} {}", source.display(), target.display());
    } else {
        fs::copy(&source, &target)?;
    }
    Ok(())
}

pub fn backup_assets(
    library: &Path,
    backup_directory: &Path,
    assets: &[Asset],
    simulate: bool,
) -> Result<(), io::Error> {
    check_path_exists_or_create(&backup_directory, simulate)?;
    let progress_bar = ProgressBar::new(assets.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    for asset in assets.iter() {
        progress_bar.inc(1);
        backup_asset(&library, &backup_directory, asset, simulate)?;
    }
    progress_bar.finish_with_message("done");
    Ok(())
}
