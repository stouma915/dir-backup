use std::ffi::OsString;
use std::fs;
use std::fs::{DirEntry, File};
use std::io::{BufReader, Read, Result};
use std::path::PathBuf;

use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use once_cell::sync::Lazy;
use regex::Regex;

const BACKUP_FILE_NAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(Mon|Tue|Wed|Thu|Fri|Sat|Sun)_(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)_[0-3][0-9]_[0-2][0-9]-[0-5][0-9]-[0-5][0-9]_[0-9]{4}.zip"
    ).unwrap()
});

pub fn current_timestamp() -> i64 {
    Local::now().timestamp()
}

pub fn parse_timestamp(millis: i64) -> String {
    let naive = NaiveDateTime::from_timestamp(millis, 0);
    let datetime = DateTime::<Local>::from_utc(naive, Local.offset_from_utc_datetime(&naive));
    let formatted = datetime.format("%a %b %0e %T %Y");

    formatted.to_string()
}

pub fn get_backup_files(path: PathBuf) -> Result<Vec<DirEntry>> {
    match fs::read_dir(path) {
        Ok(paths) => Ok(paths
            .map(|x| x.unwrap())
            .filter(|x| !x.metadata().unwrap().is_dir())
            .filter(|x| is_backup_file(x.file_name()))
            .collect()),
        Err(err) => Err(err),
    }
}

pub fn is_backup_file(os_file_name: OsString) -> bool {
    let file_name = os_file_name.into_string().unwrap();

    BACKUP_FILE_NAME_REGEX.is_match(&file_name)
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
