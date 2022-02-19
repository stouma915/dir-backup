use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::{App, Arg};
use colored::Colorize;

mod util;
mod zip_util;

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
        .arg(
            Arg::with_name("quickly")
                .help("Speed up by not compressing.")
                .long("quickly")
                .short("q"),
        )
        .get_matches();

    let source = matches.value_of("source").unwrap();
    let destination = matches.value_of("destination").unwrap();
    let quickly = matches.is_present("quickly");

    let parsed_threshold = matches.value_of("threshold").unwrap_or("50").parse::<u32>();
    if parsed_threshold.is_err() {
        println!("Please specify the threshold value numerically.");
        exit(2);
    }
    let threshold = parsed_threshold.unwrap() as usize;
    if threshold < 1 {
        println!("Please specify a threshold value of 1 or more.");
        exit(2);
    }

    if !Path::new(source).exists() || !Path::new(source).is_dir() {
        println!(
            "The source directory '{}' does not exist or isn't a directory.",
            source.yellow()
        );
        exit(2);
    }
    if !Path::new(destination).exists() || !Path::new(destination).is_dir() {
        println!(
            "The destination directory '{}' does not exist or isn't a directory.",
            destination.yellow()
        );
        exit(2);
    }

    let source_canonical = fs::canonicalize(PathBuf::from(source)).unwrap();
    let destination_canonical = fs::canonicalize(PathBuf::from(destination)).unwrap();
    if source_canonical == destination_canonical {
        println!("The source directory and destination must be different.");
        exit(2);
    }

    let mut backup_files: Vec<DirEntry> = fs::read_dir(destination)
        .unwrap()
        .map(|x| x.unwrap())
        .filter(|x| !x.metadata().unwrap().is_dir())
        .collect();
    backup_files.sort_by_key(|x| x.metadata().unwrap().created().unwrap());
    if backup_files.len() >= threshold {
        let files_to_remove = &backup_files[0..(backup_files.len() - (threshold - 1))];
        for file in files_to_remove {
            match fs::remove_file(file.path()) {
                Ok(()) => (),
                _ => (),
            }
        }

        let files: Vec<DirEntry> = fs::read_dir(destination)
            .unwrap()
            .map(|x| x.unwrap())
            .filter(|x| !x.metadata().unwrap().is_dir())
            .collect();
        if files.len() >= threshold {
            println!("The old backup couldn't be removed.");
            exit(1);
        }
    }

    let start_time = util::current_timestamp();
    println!(
        "{} {}",
        "Start:".bright_green(),
        util::parse_timestamp(start_time)
    );

    let zip_file_name = util::parse_timestamp(util::current_timestamp())
        .replace(" ", "_")
        .replace(":", "-")
        + ".zip";
    let zip_file = match fs::File::create(Path::new(destination).join(Path::new(&zip_file_name))) {
        Ok(x) => x,
        Err(err) => {
            println!("Failed to create zip: {:?}", err);
            exit(1);
        }
    };

    let entries: Vec<DirEntry> = fs::read_dir(source).unwrap().map(|x| x.unwrap()).collect();

    let zip_writer = zip::ZipWriter::new(zip_file);
    match zip_util::write_zip(zip_writer, entries, quickly) {
        Ok(mut writer) => match writer.finish() {
            Ok(_) => (),
            _ => {
                println!("The backup couldn't be created.");
                exit(1);
            }
        },
        Err(err) => {
            println!("Failed to backup: {}", err);
            exit(1);
        }
    }

    let done_time = util::current_timestamp();
    let elapsed = done_time - start_time;
    println!(
        "{} {}",
        "Done:".bright_green(),
        util::parse_timestamp(done_time)
    );
    println!("{} {}s", "Elapsed:".bright_green(), elapsed);
}
