use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use lazy_static::lazy_static;

lazy_static! {
    static ref ABOUT: String = {
        format!(
            "A iPhoto/Photo sync utility",
        )
    };
}

pub fn cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .setting(AppSettings::ColoredHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about(ABOUT.as_str())
        .arg(Arg::with_name("database")
            .short("d")
            .long("database")
            .takes_value(true)
            .required(true)
            .help("Select iPhoto database"))
}