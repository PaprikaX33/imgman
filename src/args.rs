use clap::{ArgGroup, Parser, ValueEnum};

const DEST_SUFF: &str = ".png";

#[derive(Debug)]
pub struct Args {
    mode: OperationType,
    source: String,
    dest: String,
}

impl Args {
    pub fn get_args() -> Result<Self, clap::error::Error> {
        let parsed = ParsableArgs::try_parse_from(wild::args())?;
        let mut dest = parsed.dest.trim().to_string();
        let dest = if dest.ends_with(DEST_SUFF) {
            dest
        } else {
            dest.push_str(DEST_SUFF);
            dest
        };
        let source = parsed.source.trim().to_string();
        let mode = if (parsed.gray) {
            OperationType::Gray
        } else if (parsed.gray_alp_dark) {
            OperationType::GrayAlpDark
        } else if (parsed.gray_alp) {
            OperationType::GrayAlp
        } else if (parsed.copy) {
            OperationType::Copy
        } else if (parsed.alp_to_gray) {
            OperationType::AlpToGray
        } else {
            OperationType::Gray
        };
        Ok(Self { source, dest, mode })
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
    pub fn mode(&self) -> &OperationType {
        &self.mode
    }
}

#[derive(Parser)]
#[command(name = "imgman")]
#[command(author = "Paprika")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "Simple program to manipulate image",
    long_about = "An image manipulating program for experimenting with tranparency"
)]
#[command(group(
    ArgGroup::new("mode")
        .args(&["gray", "gray_alp", "gray_alp_dark", "copy", "alp_to_gray"])
        .multiple(false)
        .required(false)
))]
pub struct ParsableArgs {
    ///Convert the image to grayscale (default)
    #[arg(long)]
    gray: bool,
    ///Convert the image to grayscale and store the image as transparency with white blank image
    #[arg(long = "gray-alpha")]
    gray_alp: bool,
    ///Convert the image to grayscale and store the image as transparency with black blank image
    #[arg(long = "gray-alpha-dark")]
    gray_alp_dark: bool,
    ///Plainly copy the image from source to destination
    #[arg(long = "copy")]
    copy: bool,
    ///Discard the image, and replace is with the image from the transparency
    #[arg(long = "alpha-to-gray")]
    alp_to_gray: bool,
    #[arg(value_name = "SOURCE")]
    #[arg(help = "Source Image")]
    source: String,
    #[arg(value_name = "DESTINATION", default_value_t = String::from("manip.png"))]
    #[arg(help = "Destination Image")]
    dest: String,
}

#[derive(Debug)]
pub enum OperationType {
    Gray,
    GrayAlp,
    GrayAlpDark,
    Copy,
    AlpToGray,
}
