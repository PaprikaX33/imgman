use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "imgman")]
#[command(author = "Paprika")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(
    about = "Simple program to manipulate image",
    long_about = "An image manipulating program for experimenting with tranparency"
)]
pub struct Args {
    #[arg(long, short, value_name = "SOURCE FILE")]
    #[arg(help = "Source File")]
    #[arg(visible_aliases=["src"])]
    source: String,
    #[arg(long, short, value_name = "DESTINATION FILE", default_value_t = String::from("dest.png"))]
    #[arg(help = "Destination File")]
    dest: String,
}

impl Args {
    pub fn get_args() -> Result<Self, clap::error::Error> {
        Self::try_parse_from(wild::args())
    }
}
