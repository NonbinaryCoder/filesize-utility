use std::{env, path::*};

#[derive(Debug)]
pub struct SearchSettings {
    pub path: PathBuf,
    pub sort_mode: SortMode,
    pub reverse_sort: bool,
    pub explore: bool,
}

impl SearchSettings {
    fn new(path: PathBuf) -> Self {
        SearchSettings {
            path,
            sort_mode: SortMode::default(),
            reverse_sort: false,
            explore: false,
        }
    }
}

#[derive(Debug)]
pub enum SortMode {
    None,
    Size,
    Lex,
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
                "-n" | "--none" => search_settings.sort_mode = SortMode::None,
                "-s" | "--size" => search_settings.sort_mode = SortMode::Size,
                "-l" | "--lex" => search_settings.sort_mode = SortMode::Lex,
                "-r" | "--reverse" => search_settings.reverse_sort = !search_settings.reverse_sort,
                "-e" | "--explore" => search_settings.explore = !search_settings.explore,
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
\t-n --none     Use order entries are returned by file system
\t-s --size     Order entries by entry size, larger entries at top (default)
\t-l --lex      Order entries lexicographically
\t-r --reverse  Reverse the ordering of entries
\t-e --explore  Launch the app in explore mode
        "
        );
        return None;
    }
}
