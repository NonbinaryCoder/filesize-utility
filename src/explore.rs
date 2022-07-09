use crate::{display_number::*, get_entries::*, parse_args::*};
use std::io::{self, Write};

static HELP_MESSAGE: &str = "\
Enter the name of a directory to go to that directory
-b --back  Go up 1 directory
-e --exit  Exit the app\
";

pub fn explore(mut settings: SearchSettings) {
    //let original_path = settings.path.clone();
    let mut input_buf = String::with_capacity(8);
    let mut entries = get_entries(&settings).expect("Unable to get entries");
    loop {
        let input = {
            eprint!("> ");
            io::stderr().flush().unwrap();
            input_buf.clear();
            let end_pos = io::stdin().read_line(&mut input_buf).unwrap();
            let input = &input_buf[0..end_pos];
            input.trim()
        };

        if input.is_empty() {
            eprintln!("{}", HELP_MESSAGE);
        } else if input.starts_with('-') {
            let input = input.to_lowercase();
            match input.as_str() {
                "-b" | "--back" => {
                    if settings.path.pop() {
                        entries = get_entries(&settings).expect("Unable to get entries");
                    } else {
                        eprintln!("Unable to go back");
                    }
                }
                "-e" | "--exit" => break,
                _ => (),
            }
        } else {
            if input.len() > 1 {
                let input: std::ffi::OsString = input.to_lowercase().into();
                let is_single_char = input.len() == 1;
                if let Some(entry) = entries
                    .entries
                    .iter()
                    .find(|entry| entry.name.to_ascii_lowercase() == input)
                {
                    match entry.typ {
                        EntryType::Dir | EntryType::Symlink => {
                            settings.path.push(&entry.name);
                            entries = get_entries(&settings).expect("Unable to get entries");
                            continue;
                        }
                        EntryType::File => {
                            if !is_single_char {
                                eprintln!("Cannot explore a file!");
                                continue;
                            }
                        }
                        EntryType::Unknown => {
                            if !is_single_char {
                                eprintln!("Cannot explore unknown file type!");
                                continue;
                            }
                        }
                    }
                }
            }
            let mut chars = input.chars();
            let c = chars.next();
            if let Some(c) = c {
                if chars.next().is_none() {
                    let num = char_to_num(c);
                    if let Some(num) = num {
                        if num < entries.entries.len() {
                            let entry = &entries.entries[num];
                            match entry.typ {
                                EntryType::Dir | EntryType::Symlink => {
                                    settings.path.push(&entry.name);
                                    entries =
                                        get_entries(&settings).expect("Unable to get entries");
                                }
                                EntryType::File => eprintln!("Cannot explore a file!"),
                                EntryType::Unknown => {
                                    eprintln!("Cannot explore unknown file type!")
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
