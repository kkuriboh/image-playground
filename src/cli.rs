use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use image::ImageFormat;

#[derive(Parser)]
pub struct Playground {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Sort {
        #[arg(long, short)]
        image: PathBuf,
        #[arg(long, short, default_value = "40,90")]
        luminance_threshold: Threshold,
        #[arg(long, short, default_value = "left-to-right")]
        kind: SortKind,
        #[arg(long, short, default_value = "horizontal")]
        direction: SortDirection,
        #[arg(long, short, default_value = "default")]
        method: SortMethod,
        #[arg(long, short, default_value = "output.png")]
        output: String,
        #[arg(long, short, default_value = "png")]
        format: Format,
    },
}

#[derive(Clone)]
pub struct Threshold {
    pub min: f32,
    pub max: f32,
}

impl From<String> for Threshold {
    fn from(value: String) -> Self {
        let mut splited = value.split(',');
        let min = splited
            .next()
            .expect("min not supplied")
            .parse()
            .expect("min is not a number");

        let max = splited
            .next()
            .expect("max not supplied")
            .parse()
            .expect("max is not a number");

        Self { min, max }
    }
}

#[derive(ValueEnum, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum SortMethod {
    Default,
    Red,
    Green,
    Blue,
    Alpha,
    Hue,
    Saturation,
    Lightness,
}

#[derive(ValueEnum, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum SortDirection {
    Vertical,
    Horizontal,
}

#[derive(ValueEnum, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum SortKind {
    LeftToRight,
    RightToLeft,
}

#[derive(ValueEnum, Clone, PartialEq, Debug, Ord, PartialOrd, Eq)]
pub enum Format {
    Png,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    OpenExr,
    Farbfeld,
    Avif,
    Qoi,
}

impl Into<ImageFormat> for Format {
    fn into(self) -> ImageFormat {
        match self {
            Self::Png => ImageFormat::Png,
            Self::Jpeg => ImageFormat::Jpeg,
            Self::Gif => ImageFormat::Gif,
            Self::WebP => ImageFormat::WebP,
            Self::Pnm => ImageFormat::Pnm,
            Self::Tiff => ImageFormat::Tiff,
            Self::Tga => ImageFormat::Tga,
            Self::Dds => ImageFormat::Dds,
            Self::Bmp => ImageFormat::Bmp,
            Self::Ico => ImageFormat::Ico,
            Self::Hdr => ImageFormat::Hdr,
            Self::OpenExr => ImageFormat::OpenExr,
            Self::Farbfeld => ImageFormat::Farbfeld,
            Self::Avif => ImageFormat::Avif,
            Self::Qoi => ImageFormat::Qoi,
        }
    }
}
