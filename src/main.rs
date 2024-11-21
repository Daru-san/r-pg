use clap::Parser;
use image;

/// Simple in-development image manipulation tool
#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    /// File name of the source image
    source: String,

    /// File name of the final image after conversion with the file type
    output: String,
}

fn main() {
    let args = Args::parse();
    let img = image::open(args.source).unwrap();
    img.save(args.output).unwrap();
}
