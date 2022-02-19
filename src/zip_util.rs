use std::fs::{DirEntry, File};

use zip::ZipWriter;

pub fn write_zip(
    mut writer: ZipWriter<File>,
    entries: Vec<DirEntry>,
) -> Result<ZipWriter<File>, String> {
    Ok(writer)
}
