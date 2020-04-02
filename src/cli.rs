use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg};
use lazy_static::lazy_static;

lazy_static! {
    static ref ABOUT: String = {
            "A iPhoto/Photo sync utility".to_string()
    };
}

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT.as_str())
        .arg(Arg::with_name("database")
            .short("f")
            .long("database")
            .takes_value(true)
            .required(true)
            .help("Select iPhoto database"))
        .arg(Arg::with_name("backup_directory")
            .short("o")
            .long("backup_directory")
            .takes_value(true)
            .required(true)
            .help("Select backup directory for iPhoto events"))
        .arg(Arg::with_name("dry_run")
            .short("n")
            .long("dry_run")
            .help("show what would have been transferred"))
}