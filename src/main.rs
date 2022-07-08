use get_entries::*;
use parse_args::*;

mod get_entries;
mod parse_args;

fn main() {
    match parse_args() {
        Some(settings) => {
            get_entries(&settings);
        }
        None => (),
    }
}
