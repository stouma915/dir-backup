use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn current_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn parse_timestamp(millis: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(millis, 0);
    let datetime = DateTime::<Local>::from_utc(naive, Local.offset_from_utc_datetime(&naive));
    let formatted = datetime.format("%a %b %e %T %Y");

    formatted.to_string()
}

pub fn get_backup_files(path: PathBuf) -> Result<Vec<DirEntry>> {
    match fs::read_dir(path) {
        Ok(paths) => Ok(paths
            .map(|x| x.unwrap())
            .filter(|x| !x.metadata().unwrap().is_dir())
            .collect()),
        Err(err) => Err(err),
    }
}

pub fn read_bytes(path: PathBuf) -> Result<Vec<u8>> {
    match File::open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer).map(|_| buffer)
        }
        Err(e) => Err(e),
    }
}
