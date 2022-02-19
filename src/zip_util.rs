use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read, Write};

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
                                    Ok(_) => println!("Complete: {}", &entry_name),
                                    _ => println!(
                                        "WARNING: Couldn't backup {}: Cannot write",
                                        &entry_name
                                    ),
                                },
                                _ => println!(
                                    "WARNING: Couldn't backup {}: Cannot read",
                                    &entry_name
                                ),
                            }
                        }
                        _ => println!("WARNING: Couldn't backup {}: Cannot open", &entry_name),
                    };
                }
                Err(err) => println!("WARNING: Couldn't backup {}: {:?}", &entry_name, err),
            }
        } else {
            match writer.add_directory(&entry_name, options) {
                Ok(_) => match fs::read_dir(&entry_name) {
                    Ok(paths) => {
                        println!("Complete: {}", &entry_name);

                        let dir_entries: Vec<DirEntry> = paths.map(|x| x.unwrap()).collect();
                        match write_zip(writer, dir_entries, quickly) {
                            Ok(w) => writer = w,
                            Err(err) => return Err(err),
                        }
                    }
                    Err(err) => println!("WARNING: Couldn't backup {}: {:?}", &entry_name, err),
                },
                Err(err) => println!("WARNING: Couldn't backup {}: {:?}", &entry_name, err),
            }
        }
    }

    Ok(writer)
}
