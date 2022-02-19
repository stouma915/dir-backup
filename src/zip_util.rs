use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read, Write};

use colored::Colorize;
use zip::ZipWriter;

pub fn write_zip(
    mut writer: ZipWriter<File>,
    entries: Vec<DirEntry>,
    quickly: bool,
) -> Result<ZipWriter<File>, String> {
    let method = if quickly {
        zip::CompressionMethod::Stored
    } else {
        zip::CompressionMethod::Bzip2
    };
    let options = zip::write::FileOptions::default().compression_method(method);

    for entry in entries {
        let entry_name = String::from(entry.path().as_path().to_str().unwrap());

        if !entry.metadata().unwrap().is_dir() {
            match writer.start_file(&entry_name, options) {
                Ok(_) => {
                    match File::open(entry.path()) {
                        Ok(file) => {
                            let mut reader = BufReader::new(file);
                            let mut buffer = Vec::new();

                            match reader.read_to_end(&mut buffer) {
                                Ok(_) => match writer.write(&*buffer) {
                                    Ok(_) => {
                                        println!("{} {}", "Complete:".bright_blue(), &entry_name)
                                    }
                                    _ => println!(
                                        "{} Couldn't backup {}: Cannot write",
                                        "WARNING:".yellow(),
                                        &entry_name
                                    ),
                                },
                                _ => println!(
                                    "{} Couldn't backup {}: Cannot read",
                                    "WARNING:".yellow(),
                                    &entry_name
                                ),
                            }
                        }
                        _ => println!(
                            "{} Couldn't backup {}: Cannot open",
                            "WARNING:".yellow(),
                            &entry_name
                        ),
                    };
                }
                Err(err) => println!(
                    "{} Couldn't backup {}: {:?}",
                    "WARNING:".yellow(),
                    &entry_name,
                    err
                ),
            }
        } else {
            match writer.add_directory(&entry_name, options) {
                Ok(_) => match fs::read_dir(&entry_name) {
                    Ok(paths) => {
                        println!("{} {}", "Complete:".bright_blue(), &entry_name);

                        let dir_entries: Vec<DirEntry> = paths.map(|x| x.unwrap()).collect();
                        match write_zip(writer, dir_entries, quickly) {
                            Ok(w) => writer = w,
                            Err(err) => return Err(err),
                        }
                    }
                    Err(err) => println!(
                        "{} Couldn't backup {}: {:?}",
                        "WARNING:".yellow(),
                        &entry_name,
                        err
                    ),
                },
                Err(err) => println!(
                    "{} Couldn't backup {}: {:?}",
                    "WARNING:".yellow(),
                    &entry_name,
                    err
                ),
            }
        }
    }

    Ok(writer)
}
