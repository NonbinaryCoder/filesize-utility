use crate::parse_args::SearchSettings;
use std::{ffi::OsString, fs, io};

#[derive(Debug)]
pub struct EntryData {
    pub name: OsString,
    pub typ: EntryType,
    pub size: Size,
}

#[derive(Debug)]
pub enum EntryType {
    Unknown,
    Dir,
    File,
    Symlink,
}

impl From<io::Result<fs::FileType>> for EntryType {
    fn from(typ: io::Result<fs::FileType>) -> Self {
        match typ {
            Ok(typ) => {
                if typ.is_dir() {
                    EntryType::Dir
                } else if typ.is_file() {
                    EntryType::File
                } else if typ.is_symlink() {
                    EntryType::Symlink
                } else {
                    EntryType::Unknown
                }
            }
            Err(_) => EntryType::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct Size;

pub fn get_entries(settings: SearchSettings) -> Option<Vec<EntryData>> {
    let entries = fs::read_dir(settings.path);
    match entries {
        Ok(entries) => {
            let mut res = Vec::with_capacity(entries.size_hint().0);
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        res.push(EntryData {
                            name: entry.file_name(),
                            typ: entry.file_type().into(),
                            size: Size,
                        });
                    }
                    Err(e) => {
                        eprintln!("Couldn't read entry: {}", e);
                        return None;
                    }
                }
            }
            return Some(res);
        }
        Err(e) => {
            eprintln!("Couldn't read path: {}", e);
            return None;
        }
    }
}
