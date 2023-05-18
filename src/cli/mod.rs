use anyhow::Error;
use clap::Parser;

mod convert;
mod dump;

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Parser, Debug)]
enum Command {
    Convert(convert::Args),
    Dump(dump::Args),
}

pub fn execute() -> Result<(), Error> {
    let args = Args::parse();
    let cmd = if let Some(cmd) = args.command {
        cmd
    } else {
        unreachable!()
    };

    match cmd {
        Command::Convert(cmd) => convert::execute(cmd),
        Command::Dump(cmd) => dump::execute(cmd),
    }
}
