use clap::Parser;
use imgcpr::{compress, decompress, PaletteMethod};
use libflate::deflate::{Decoder, Encoder};
use std::io::{Read, Write};
use std::path::PathBuf;

/// Compress or decompress image files with imgcpr format
#[derive(Debug, Parser)]
struct Cli {
    /// Path to the image file
    path: PathBuf,
    /// Flag for compression
    #[arg(action, short = 'c', long = "compress")]
    compress: bool,
    /// Output path
    #[arg(short = 'o', long = "output")]
    output: Option<PathBuf>,
    /// Palette selection method
    #[arg(value_enum,
        short = 'p',
        long = "palette",
        default_value_t = PaletteMethod::Freq)]
    palette: PaletteMethod,
    /// Debug mode
    #[arg(action, short = 'd', long = "debug")]
    debug: bool,
}

// TODO: try png- or qoi-like compression on index data
// Deflate performs best, at 122.1 KB for bright-colors
fn main() {
    let args = Cli::parse();

    if args.debug {
        println!("Args: {:#?}", args);
        let output = args.output.unwrap_or_else(|| {
            let mut path = args.path.clone();
            let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
            path.set_file_name(name + ".debug.png");
            path
        });

        let img = image::open(args.path).unwrap().into_rgb8();
        let bytes = compress::compress(&img, args.palette);

        let img = decompress::decompress(&bytes);
        println!("Saving image to: {:?}", output);
        img.save(output).unwrap();

        return;
    }

    if args.compress {
        let output = args.output.unwrap_or_else(|| {
            let mut path = args.path.clone();
            path.set_extension("imgcpr");
            path
        });

        let img = image::open(args.path).unwrap().into_rgb8();
        let bytes = compress::compress(&img, args.palette);

        let mut encoder = Encoder::new(Vec::new());
        encoder.write_all(&bytes).unwrap();
        let bytes = encoder.finish().into_result().unwrap();

        std::fs::write(output, bytes).unwrap();
    } else {
        let output = args
            .output
            .expect("Output path is required for decompression");

        let bytes = std::fs::read(args.path).unwrap();

        let mut decoder = Decoder::new(&bytes[..]);
        let mut bytes = Vec::new();
        _ = decoder.read_to_end(&mut bytes).unwrap();

        let img = decompress::decompress(&bytes);
        img.save(output).unwrap();
    }
}
