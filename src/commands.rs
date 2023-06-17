use std::convert::TryFrom;
use std::fs;

use crate::{Result};
use crate::args::*;
use crate::chunk::Chunk;
use crate::png::Png;

pub fn encode(args: EncodeArgs) -> Result<()> {
    let input = fs::read(&args.input_file_path)?;
    let output = args.output_file_path.unwrap_or(args.input_file_path);
    
    let mut png = Png::try_from(input.as_slice())?;
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);

    fs::write(output, png.as_bytes())?;
    println!("Chunk written successfully.");
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    let input = fs::read(&args.file_path)?;
    let png = Png::try_from(input.as_slice())?;
    let chunk = png.chunk_by_type(args.chunk_type.to_string().as_str());
    if let Some(c) = chunk {
        println!("Chunk : {}", c);
        println!("Chunk data : {}", c.data_as_string().unwrap_or("{Non UTF-8 data}".to_string()));
    }
    Ok(())
}

pub fn remove(args: RemoveArgs) -> crate::Result<()> {
    let input = fs::read(&args.file_path)?;
    let mut png = Png::try_from(input.as_slice())?;
    let chunk = png.remove_chunk(args.chunk_type.to_string().as_str())?;
    fs::write(&args.file_path, png.as_bytes())?;
    println!("Removed chunk: {chunk}");
    Ok(())
}

pub fn print(args: PrintArgs) -> crate::Result<()> {
    let input = fs::read(&args.file_path)?;
    let png = Png::try_from(input.as_slice())?;
    for chunk in png.chunks() {
        println!("{chunk}");
    }
    Ok(())
}