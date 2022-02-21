use std::fs;
use std::fs::{DirEntry, File};
use std::io::{Error, Write};

use colored::Colorize;
use zip::ZipWriter;

use crate::util;

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
        let entry_name = entry.path().into_os_string().into_string().unwrap();

        if !entry.metadata().unwrap().is_dir() {
            match writer.start_file(&entry_name, options) {
                Ok(_) => match util::read_bytes(entry.path()) {
                    Ok(buffer) => match writer.write(&*buffer) {
                        Ok(_) => {
                            println!("{} {}", "Complete:".bright_blue(), &entry_name)
                        }
                        Err(err) => skip_warn(&entry_name, err),
                    },
                    Err(err) => skip_warn(&entry_name, err),
                },
                Err(err) => {
                    println!(
                        "{} Skipping {} ({:?})",
                        "WARNING:".yellow(),
                        &entry_name,
                        err
                    )
                }
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
                    Err(err) => skip_warn(&entry_name, err),
                },
                Err(err) => {
                    println!(
                        "{} Skipping {} ({:?})",
                        "WARNING:".yellow(),
                        &entry_name,
                        err
                    )
                }
            }
        }
    }

    Ok(writer)
}

fn skip_warn(entry_name: &String, error: Error) -> () {
    println!(
        "{} Skipping {} ({:?})",
        "WARNING:".yellow(),
        &entry_name,
        error
    )
}
