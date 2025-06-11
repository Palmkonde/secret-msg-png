use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme", about = "PNGMe CLI Tool")]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Print(PrintArgs),
    Remove(RemoveArgs),
}

#[derive(Debug, StructOpt)]
pub struct EncodeArgs {
    /// Input Png file path
    #[structopt(short, long)]
    pub input: PathBuf,
    
    /// Chunk type to encode
    #[structopt(short = "c", long = "chunk-type")]
    pub chunk_type: String,
    
    /// Secret message to encode
    #[structopt(short)]
    pub secret: String,
    
    /// Output file path
    #[structopt(short, long)]
    pub output: Option<PathBuf>,
        
    /// index of the chunk to insert the secret message
    #[structopt(long = "index")]
    pub index: Option<usize>
}

#[derive(Debug, StructOpt)]
pub struct  DecodeArgs {
    /// Input Png file path
    #[structopt(short, long)]
    pub input: PathBuf,
    
    /// Chunk type to encode
    #[structopt(short = "c", long = "chunk-type")]
    pub chunk_type: String,
}

#[derive(Debug, StructOpt)]
pub struct PrintArgs {
    /// Input Png file path
    #[structopt(short, long)]
    pub input: PathBuf   
}

#[derive(Debug, StructOpt)]
pub struct RemoveArgs {
    /// Input Png file path
    #[structopt(short, long)]
    pub input: PathBuf,
    
    /// Chunk type to remove
    #[structopt(short = "c", long = "chunk-type")]
    pub chunk_type: String,
}
