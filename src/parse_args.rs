use std::{env, path::*};

#[derive(Debug)]
pub struct SearchSettings {
    pub path: PathBuf,
    pub sort_mode: SortMode,
}

impl SearchSettings {
    fn new(path: PathBuf) -> Self {
        SearchSettings {
            path,
            sort_mode: SortMode::default(),
        }
    }
}

#[derive(Debug)]
pub enum SortMode {
    Size,
    Lexicographic,
    RevSize,
    RevLex,
}

impl Default for SortMode {
    fn default() -> Self {
        SortMode::Size
    }
}

pub fn parse_args() -> Option<SearchSettings> {
    let mut args = env::args();
    // Executable path.  Skip
    args.next();

    if let Some(path_string) = args.next() {
        let path = PathBuf::from(path_string);
        let mut search_settings = SearchSettings::new(path);
        for option in args {
            match &option[..] {
                "-s" | "--size" => search_settings.sort_mode = SortMode::Size,
                "-l" | "--lex" => search_settings.sort_mode = SortMode::Lexicographic,
                "-r" | "--rev-size" => search_settings.sort_mode = SortMode::RevSize,
                "-v" | "--rev-lex" => search_settings.sort_mode = SortMode::RevLex,
                _ => (),
            }
        }
        return Some(search_settings);
    } else {
        eprintln!(
            "
Usage:
\tfilesize-utility <path> [options]

Options:
\t-s --size      Sort by file size, larger files at top (default)
\t-l --lex       Sort lexicographically
\t-r --rev-size  Sort by file size, larger files at bottom
\t-v --rev-lex   Sort reverse lexicographically\
        "
        );
        return None;
    }
}
