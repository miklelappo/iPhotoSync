mod db;

use db::Asset;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = "/tmp/Photos.sqlite";
    db::get_db_assets(&path).expect("Failed to get assets from DB");
    Ok(())
}
