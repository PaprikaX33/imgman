mod args;
mod image;
mod operation;
use args::OperationType;
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
    let mut img = Image::open(config.source())?;
    let oper = match config.mode() {
        OperationType::Gray => operation::grayscale,
        OperationType::GrayAlp => |x| operation::grayscale_to_alph(x, 0xff),
        OperationType::GrayAlpDark => |x| operation::grayscale_to_alph(x, 0x00),
        OperationType::Copy => |x| x,
        OperationType::AlpToGray => operation::grayscale_from_alph,
    };
    let _ = img.apply_per_pixel(oper);
    img.write(config.dest())?;
    Ok(ExitCode::SUCCESS)
}
