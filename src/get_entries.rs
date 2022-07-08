use crate::parse_args::SearchSettings;
use byte_unit::*;
use std::{ffi::OsString, fmt::Display, fs, io, path::*};

#[derive(Debug)]
pub struct EntryVec {
    pub path: PathBuf,
    pub entries: Vec<EntryData>,
}

impl Display for EntryVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let longest = {
            let mut longest = 0;
            for entry in self.entries.iter() {
                longest = longest.max(entry.name.len());
            }
            longest
        };

        for entry in self.entries.iter() {
            let name = entry.name.to_string_lossy();
            let typ = &entry.typ;
            let size = &entry.size;
            let size = format!("{}", size);
            writeln!(f, "({typ}) {name:longest$} {size:>9}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct EntryData {
    pub name: OsString,
    pub typ: EntryType,
    pub size: Size,
}

impl Display for EntryData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.to_string_lossy();
        let typ = &self.typ;
        let size = &self.size;
        write!(f, "({typ}) {name} {size}")
    }
}

#[derive(Debug)]
pub enum EntryType {
    Unknown,
    Dir,
    File,
    Symlink,
}

impl Display for EntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryType::Unknown => write!(f, "?"),
            EntryType::Dir => write!(f, "D"),
            EntryType::File => write!(f, "F"),
            EntryType::Symlink => write!(f, "L"),
        }
    }
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
pub enum Size {
    Known(AdjustedByte),
    Unknown,
}

impl From<Result<u64, fs_extra::error::Error>> for Size {
    fn from(size: Result<u64, fs_extra::error::Error>) -> Self {
        match size {
            Ok(size) => {
                let size = Byte::from_bytes(size);
                let size = size.get_appropriate_unit(false);
                Size::Known(size)
            }
            Err(_) => Size::Unknown,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Known(size) => {
                write!(f, "{size}")
            }
            Size::Unknown => write!(f, "unknown"),
        }
    }
}

pub fn get_entries(settings: &SearchSettings) -> Option<EntryVec> {
    let entries = fs::read_dir(&settings.path);
    match entries {
        Ok(entries) => {
            let mut res = EntryVec {
                path: settings.path.clone(),
                entries: Vec::with_capacity(entries.size_hint().0),
            };
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        res.entries.push(EntryData {
                            name: entry.file_name(),
                            typ: entry.file_type().into(),
                            size: fs_extra::dir::get_size(entry.path()).into(),
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
