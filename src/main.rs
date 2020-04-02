#![allow(non_snake_case)]

mod db;
mod cli;
mod fs_utils;

use db::Asset;
use std::io;
use indicatif::{ ProgressBar, ProgressStyle };

fn main() -> Result<(), io::Error> {
    let args = cli::cli().get_matches();
    let mut backup_table: Vec<Asset> = Vec::new();
    let backup_directory: String = String::from(args.value_of("backup_directory").unwrap());
    let library: String = String::from(args.value_of("database").unwrap());
    let simulate: bool = args.is_present("dry_run");

    fs_utils::check_path_exists_or_create(&backup_directory)?;
    backup_table = db::get_db_assets(&library, backup_table)
        .expect("Failed to get assets from DB");

    let progress_bar = ProgressBar::new(backup_table.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .progress_chars("##-"));

    for asset in backup_table.iter() {
        progress_bar.inc(1);
        fs_utils::backup_asset(&library, &backup_directory, &asset, simulate)?;
    }
    progress_bar.finish_with_message("done");
    Ok(())
}
