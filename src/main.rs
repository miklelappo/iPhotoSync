#![allow(non_snake_case)]

mod cli;
mod db;
mod fs_utils;

use fs_utils::backup_assets;
use std::io;
use std::path::PathBuf;

fn main() -> Result<(), io::Error> {
    let args = cli::cli().get_matches();
    let backup_directory = PathBuf::from(args.value_of("backup_directory").unwrap());
    let library = PathBuf::from(args.value_of("database").unwrap());
    let dry_run = args.is_present("dry_run");

    let backup_table = db::get_db_assets(&library).expect("Failed to get assets from DB");

    backup_assets(&library, &backup_directory, &backup_table, dry_run)
}
