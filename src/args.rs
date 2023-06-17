use clap::{Parser,Subcommand,Args};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use crate::chunk_type::ChunkType;

#[derive(Parser,Debug)]
#[command(version="1.0", about = "Hide messages in a PNG File", long_about = None)]
pub struct Arg{
    #[clap(subcommand)]
    pub subcommand: SubcommandType,
}

#[derive(Debug, Subcommand)]
pub enum SubcommandType {
    /// Hide message in a PNG File.   
    Encode(EncodeArgs),

    /// Decode hidden message from a PNG File.
    Decode(DecodeArgs),

    /// Remove the hidden message from a PNG File.
    Remove(RemoveArgs),

    /// Print all chunks in a PNG File.
    Print(PrintArgs),
}


#[derive(Args,Debug,PartialEq)]
pub struct EncodeArgs {
    /// Input PNG File path
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub input_file_path: PathBuf,

    /// Chunk Type [4-Byte value made up of a-z | A-Z]
    #[arg(value_parser=clap::builder::ValueParser::new(parse_chunk_type))]
    pub chunk_type: ChunkType,

    /// Message to hide
    pub message: String,

    /// [Optional] Output file path, If not given message will be written to input file 
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub output_file_path: Option<PathBuf>,
}

#[derive(Args,Debug)]
pub struct DecodeArgs {
    /// PNG File path
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub file_path: PathBuf,

    /// Chunk Type [4-Byte value made up of a-z | A-Z]
    #[arg(value_parser=clap::builder::ValueParser::new(parse_chunk_type))]
    pub chunk_type: ChunkType,
}


#[derive(Args,Debug)]
pub struct RemoveArgs {
    /// PNG File path
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub file_path: PathBuf,

    /// Chunk Type [4-Byte value made up of a-z | A-Z]
    #[arg(value_parser=clap::builder::ValueParser::new(parse_chunk_type))]
    pub chunk_type: ChunkType,
}


#[derive(Args,Debug)]
pub struct PrintArgs {
    /// PNG File path
    #[arg(value_parser=clap::value_parser!(PathBuf))]
    pub file_path: PathBuf,
}

fn parse_chunk_type(env: &str)-> Result<ChunkType,std::io::Error>{
    let chunk_type = ChunkType::from_str(env);
    if let Err(_) = chunk_type{
        eprintln!("Couldnot parse chunk type");
        exit(1);
    }
    Ok(chunk_type.unwrap())
}

