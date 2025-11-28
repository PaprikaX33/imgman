mod args;
mod image;
use image::Image;
use std::process::ExitCode;

fn main() -> anyhow::Result<ExitCode> {
    let config = match args::Args::get_args() {
        Ok(x) => x,
        Err(err) => {
            if err.exit_code() == 0 {
                println!("{}", err);
                return Ok(ExitCode::SUCCESS);
            } else {
                return Err(err.into());
            }
        }
    };
    println!("{:?}", config);
    let _ = Image::open(config.source())?;
    Ok(ExitCode::SUCCESS)
}
