use std::str::FromStr;
use structopt::StructOpt;

use crate::commands::Cli;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::from_args();
    
    match cli.command {
        args::PngMeArgs::Encode(args) => {
            let input = args.input;
            let chunk_type = ChunkType::from_str(&args.chunk_type).unwrap(); 
            let secret_message = args.secret.into_bytes();
            let mut png = Png::from_file(&input).unwrap();

            match args.index {
                Some(idx) => png.insert_chunk(idx, Chunk::new(chunk_type, secret_message.clone())),
                None => png.append_chunk(Chunk::new(chunk_type, secret_message.clone()))           
            };
            
            let output_path = match args.output {
                Some(path) => path,
                None => input.with_extension("png")
            };
            
            png.save(&output_path)
                .map_err(|e| format!("Failed to save PNG file: {}", e))?;
            
            println!("Successfully encoded message into PNG file: {}", output_path.display());
        }
        
        args::PngMeArgs::Decode(args) => {
            let input = args.input;
            let chunk_type = args.chunk_type;
            
            let png = Png::from_file(&input).unwrap();
            
            match png.chunk_by_type(&chunk_type) {
                Some(chunk) => {
                    let secret_message = chunk.data();
                    println!("Decoded message: {}", String::from_utf8_lossy(secret_message));
                }
                None => {
                    eprintln!("No chunk of type '{}' found in the PNG file.", chunk_type);
                }
            }
        }
        
        args::PngMeArgs::Print(args) => {
            let input = args.input;
            let png = Png::from_file(&input).unwrap();
            
            println!("This is all chunks and their types:");
            for (i, chunk) in png.chunks().iter().enumerate() {
                println!("{}. Chunk Type: {}, Length: {}", i, chunk.chunk_type(), chunk.data().len());

            }
        }
        
        args::PngMeArgs::Remove(args) => {
            let input = args.input;
            let chunk_type = args.chunk_type;
            
            let mut png = Png::from_file(&input).unwrap();
            
            png.remove_first_chunk(&chunk_type)
                .map_err(|e| format!("Failed to remove chunk: {}", e))?;            
            
            png.save(&input)
                .map_err(|e| format!("Failed to save PNG file: {}", e))?;
            println!("Removed first chunk of type '{}'", chunk_type);
        }
    }

    Ok(())
}

