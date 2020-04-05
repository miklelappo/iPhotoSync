#![allow(non_snake_case)]

mod db;
mod cli;
mod fs_utils;

use db::Asset;
use std::io;
use fs_utils::{ backup_assets };

fn main() -> Result<(), io::Error> {
    let args = cli::cli().get_matches();
    let mut backup_table: Vec<Asset> = Vec::new();
    let backup_directory: String = String::from(args.value_of("backup_directory").unwrap());
    let library: String = String::from(args.value_of("database").unwrap());
    let simulate: bool = args.is_present("dry_run");

    backup_table = db::get_db_assets(&library, backup_table)
        .expect("Failed to get assets from DB");

    backup_assets(&library, &backup_directory, &backup_table, simulate)
        .expect("Failed to backup assets");

    Ok(())
}
