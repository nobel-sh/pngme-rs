// mod args;
mod args;
mod chunk_type;
mod chunk;
mod commands;
mod png;

use clap::{Parser};
use crate::args::{Arg,SubcommandType};
use commands::{encode,decode,print,remove};

//custom error and result type
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Arg::parse();

    let _ = match args.subcommand {
        SubcommandType::Encode(args) => encode(args),
        SubcommandType::Decode(args) => decode(args),
        SubcommandType::Remove(args) => remove(args),
        SubcommandType::Print(args) => print(args),
    };
    Ok(())
}