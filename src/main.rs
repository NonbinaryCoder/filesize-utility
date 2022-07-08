use parse_args::*;

mod parse_args;

fn main() {
    match parse_args() {
        Some(settings) => {
            eprintln!("{:?}", settings);
        }
        None => (),
    }
}
