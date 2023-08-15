use std::{fs::File, io::BufWriter};

use clap::Parser;
use effects::sort::sort;
use image::ImageFormat;

mod cli;
mod color_helpers;
mod effects;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let args = cli::Playground::parse();

    let cli::Commands::Sort {
        image,
        luminance_threshold,
        kind,
        direction,
        method,
        output,
        format,
    } = args.command;

    let img = image::open(image)?;

    let new_img = sort(img, luminance_threshold, kind, direction, method);

    let mut out = BufWriter::new(File::create(output).unwrap());
    new_img
        .write_to::<_, ImageFormat>(&mut out, format.into())
        .unwrap();

    Ok(())
}
