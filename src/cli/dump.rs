use anyhow::Error;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Parser, Debug)]
pub struct Args {
    // The file to dump
    #[arg(short, long)]
    file: String,
}

pub fn execute(cmd: Args) -> Result<(), Error> {
    let f = File::open(cmd.file.clone())?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    for value in buffer {
        println!("BYTE: {}", value);
    }

    println!("{:?}", cmd);

    Ok(())
}
