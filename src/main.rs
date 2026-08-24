use std::{env, process::ExitCode};

fn main() -> ExitCode {
    match bullet_family::cli::run(env::args_os(), env::current_dir()) {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("bullet-family: {error}");
            ExitCode::from(error.exit_code())
        }
    }
}
