use crate::parse_args::SearchSettings;
use byte_unit::*;
use crossterm::{self, cursor::*, execute, style::*};
use std::{
    ffi::OsString,
    fmt::Display,
    fs,
    io::{self, Write},
    path::*,
};

#[derive(Debug)]
pub struct EntryVec {
    pub path: PathBuf,
    pub entries: Vec<EntryData>,
}

impl EntryVec {
    /// Overwrites what this had previously displayed with it's new structure.  Assumes the cursor hasn't moved and the
    /// elements of this haven't changed, only reordered
    fn reprint(&self) {
        execute!(
            io::stdout(),
            MoveUp(
                self.entries
                    .len()
                    .try_into()
                    .expect("You have more than u16::MAX files in this directory!  Wow!")
            ),
            MoveToColumn(0),
        )
        .unwrap();
        let longest = {
            let mut longest = 0;
            for entry in self.entries.iter() {
                longest = longest.max(entry.name.len());
            }
            longest
        };
        for entry in self.entries.iter() {
            entry.print_with_padding(longest);
        }
    }
}

#[derive(Debug)]
pub struct EntryData {
    pub name: OsString,
    pub path: PathBuf,
    pub typ: EntryType,
    pub size: Size,
}

impl EntryData {
    fn print_with_padding(&self, padding: usize) {
        let name = self.name.to_string_lossy();
        let typ = &self.typ;
        let size = &self.size;
        execute!(
            io::stdout(),
            SetForegroundColor(typ.get_color()),
            Print(format!("({typ})")),
            ResetColor,
            Print(format!(" {name:padding$} ")),
            SetForegroundColor(size.get_color()),
            Print(format!("{:>9}\n", format!("{}", size))),
            ResetColor,
        )
        .unwrap();
    }
}

#[derive(Debug)]
pub enum EntryType {
    Unknown,
    Dir,
    File,
    Symlink,
}

impl EntryType {
    fn get_color(&self) -> crossterm::style::Color {
        match self {
            EntryType::Unknown => Color::DarkRed,
            EntryType::Dir => Color::White,
            EntryType::File => Color::Green,
            EntryType::Symlink => Color::Blue,
        }
    }
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

impl Size {
    fn get_color(&self) -> crossterm::style::Color {
        match self {
            Size::Known(size) => match size.get_unit() {
                ByteUnit::B => Color::DarkGrey,
                ByteUnit::KB => Color::DarkGreen,
                ByteUnit::MB => Color::DarkYellow,
                ByteUnit::GB => {
                    if size.get_value() < 10.0 {
                        Color::DarkRed
                    } else if size.get_value() < 25.0 {
                        Color::Red
                    } else {
                        Color::Magenta
                    }
                }
                _ => Color::Blue,
            },
            Size::Unknown => Color::Blue,
        }
    }
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
            let mut longest = 0;
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let name = entry.file_name();
                        longest = longest.max(name.len());
                        res.entries.push(EntryData {
                            name,
                            path: entry.path(),
                            typ: entry.file_type().into(),
                            size: Size::Unknown,
                        });
                    }
                    Err(e) => {
                        eprintln!("Couldn't read entry: {}", e);
                        return None;
                    }
                }
            }
            let longest = longest;
            clear_terminal();
            println!("{}:", res.path.to_string_lossy());
            for entry in res.entries.iter_mut() {
                let typ = &entry.typ;
                let name = entry.name.to_string_lossy();
                execute!(
                    io::stdout(),
                    SetForegroundColor(typ.get_color()),
                    Print(format!("({typ})")),
                    ResetColor,
                    Print(format!(" {name:longest$} ")),
                )
                .unwrap();
                io::stdout().flush().unwrap();
                entry.size = fs_extra::dir::get_size(&entry.path).into();
                execute!(
                    io::stdout(),
                    SetForegroundColor(entry.size.get_color()),
                    Print(format!("{:>9}\n", format!("{}", entry.size))),
                    ResetColor,
                )
                .unwrap();
            }
            return Some(res);
        }
        Err(e) => {
            eprintln!("Couldn't read path: {}", e);
            return None;
        }
    }
}

fn clear_terminal() {
    print!("{}[2J", 27 as char);
    execute!(io::stdout(), MoveTo(0, 0),).unwrap();
}
