use clap::Parser;
use imgcpr::{compress, decompress};
use std::path::PathBuf;

/// Compress or decompress image files with imgcpr format
#[derive(Debug, Parser)]
struct Cli {
    /// Path to the image file
    path: PathBuf,
    /// Flag for compression
    #[arg(short = 'c', long = "compress", action)]
    compress: bool,
    /// Output path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if args.compress {
        let output = args.output.unwrap_or_else(|| {
            let mut path = args.path.clone();
            path.set_extension("imgcpr");
            path
        });

        let img = image::open(args.path).unwrap();
        let img = img.into_rgba8();
        let bytes = compress::compress(&img);
        std::fs::write(output, bytes).unwrap();
    } else {
        let output = args
            .output
            .expect("Output path is required for decompression");

        let bytes = std::fs::read(args.path).unwrap();
        let img = decompress::decompress(&bytes);
        img.save(output).unwrap();
    }
}
