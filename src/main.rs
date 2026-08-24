use std::{env, process::ExitCode};

fn main() -> ExitCode {
    match bullet_family::cli::execute(env::args_os(), env::current_dir()) {
        Ok(outcome) => {
            println!("{}", outcome.output());
            ExitCode::from(outcome.exit_code())
        }
        Err(error) => {
            eprintln!("bullet-family: {error}");
            ExitCode::from(error.exit_code())
        }
    }
}
