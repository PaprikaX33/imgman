use clap::Parser;

const DEST_SUFF: &str = ".png";

#[derive(Parser, Debug)]
#[command(name = "imgman")]
#[command(author = "Paprika")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "Simple program to manipulate image",
    long_about = "An image manipulating program for experimenting with tranparency"
)]
pub struct Args {
    #[arg(value_name = "SOURCE")]
    #[arg(help = "Source Image")]
    source: String,
    #[arg(value_name = "DESTINATION", default_value_t = String::from("manip.png"))]
    #[arg(help = "Destination Image")]
    dest: String,
}

impl Args {
    pub fn get_args() -> Result<Self, clap::error::Error> {
        let parsed = Self::try_parse_from(wild::args())?;
        let mut dest = parsed.dest.trim().to_string();
        let dest = if dest.ends_with(DEST_SUFF) {
            dest
        } else {
            dest.push_str(DEST_SUFF);
            dest
        };
        let source = parsed.source.trim().to_string();
        Ok(Self { source, dest })
    }
    pub fn source(&self) -> &str {
        &self.source
    }
    pub fn dest(&self) -> &str {
        &self.dest
    }
}
