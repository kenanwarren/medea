use std::process;

mod cli;

pub fn main() {
    let result = cli::execute();
    let status = match result {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Error: {:?}", err);
            1
        }
    };

    process::exit(status);
}
