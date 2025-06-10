
use structopt::StructOpt;

use crate::args::PngMeArgs;

#[derive(StructOpt, Debug)]
#[structopt(name = "pngme", about = "PNGMe CLI Tool for encoding and decoding PNG files")]
pub struct Cli {
    #[structopt(subcommand)]
    pub command: PngMeArgs,
}