mod db;
mod cli;

use db::Asset;
use std::io;
use indicatif::{ ProgressBar, ProgressStyle };
use std::thread;
use std::time::Duration;

fn main() -> Result<(), io::Error> {
    let args = cli::cli().get_matches();
    let mut backup_table: Vec<Asset> = Vec::new();
    backup_table = db::get_db_assets(args.value_of("database").unwrap(), backup_table)
        .expect("Failed to get assets from DB");
    let pb = ProgressBar::new(backup_table.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                    .progress_chars("##-"));

    for asset in backup_table.iter() {
        pb.inc(1);
        thread::sleep(Duration::from_millis(1000));
    }
    pb.finish_with_message("done");
    Ok(())
}
