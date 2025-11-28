mod args;
mod pixel;
use pixel::Pix;
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
    Ok(ExitCode::SUCCESS)
}
