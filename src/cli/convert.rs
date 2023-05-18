use anyhow::Error;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    // The file to convert
    #[arg(short, long)]
    file: String,
}

pub fn execute(cmd: Args) -> Result<(), Error> {
    println!("{:?}", cmd);

    Ok(())
}
