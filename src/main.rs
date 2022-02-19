use std::process::exit;

use clap::{App, Arg};

mod util;

fn main() {
    let matches = App::new("dir-backup")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))
        .arg(
            Arg::with_name("source")
                .help("Directory to backup.")
                .long("source")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("destination")
                .help("Backup destination.")
                .long("destination")
                .short("d")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("threshold")
                .help("Number of backup files to keep(Default: 50)")
                .long("threshold")
                .short("t")
                .takes_value(true),
        )
        .get_matches();

    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();

    let parsed_threshold = matches.value_of("threshold").unwrap_or("50").parse::<u32>();
    if parsed_threshold.is_err() {
        println!("Please specify the threshold value numerically.");
        exit(2);
    }
    let threshold = parsed_threshold.unwrap();
    if threshold < 1 {
        println!("Please specify a threshold value of 1 or more.");
        exit(2);
    }

    let start_time = util::current_timestamp();
    println!("Start: {}", util::parse_timestamp(start_time))
}
