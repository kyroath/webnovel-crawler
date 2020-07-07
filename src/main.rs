#[macro_use]
extern crate log;

use clap::{crate_authors, crate_description, crate_version, App, Arg};

mod cli;
mod info;

fn main() {
    let matches = App::new("Webnovel Crawler")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .help("Sets the level of verbosity")
                .multiple(true),
        )
        .arg(
            Arg::with_name("URL")
                .short("u")
                .long("URL")
                .takes_value(true)
                .help("URL of the webnovel"),
        )
        .get_matches();

    match matches.occurrences_of("v") {
        1 => std::env::set_var("RUST_LOG", "webnovel_crawler=warn"),
        2 => std::env::set_var("RUST_LOG", "webnovel_crawler=info"),
        3 => std::env::set_var("RUST_LOG", "webnovel_crawler=debug"),
        4 => std::env::set_var("RUST_LOG", "webnovel_crawler=trace"),
        _ => std::env::set_var("RUST_LOG", "webnovel_crawler=error"),
    }

    pretty_env_logger::init();
    trace!("Starting application");

    let url = matches.value_of("URL");
    cli::init_console(url);
}
