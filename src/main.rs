use explore::*;
use get_entries::*;
use parse_args::*;

mod display_number;
mod explore;
mod get_entries;
mod parse_args;

fn main() {
    match parse_args() {
        Some(settings) => {
            if settings.explore {
                explore(settings);
            } else {
                get_entries(&settings);
            }
        }
        None => (),
    }
}
