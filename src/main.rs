mod background;
mod calculator;
mod catalog;
mod cli;
mod dismissal;
mod files;
mod platform;
mod search;
mod ui;
mod window_drag;

fn main() -> std::process::ExitCode {
    let command = match cli::Command::parse(std::env::args_os().skip(1)) {
        Ok(command) => command,
        Err(message) => {
            eprintln!("{message}\n\n{}", cli::USAGE);
            return std::process::ExitCode::from(2);
        }
    };

    if command == cli::Command::Help {
        println!("{}", cli::USAGE);
        return std::process::ExitCode::SUCCESS;
    }

    platform::run().into()
}
