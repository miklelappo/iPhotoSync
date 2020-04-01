mod db;
mod cli;

use db::Asset;
use std::io;

fn main() -> Result<(), io::Error> {
    let args = cli::cli().get_matches();
    db::get_db_assets(args.value_of("database").unwrap()).expect("Failed to get assets from DB");
    Ok(())
}
