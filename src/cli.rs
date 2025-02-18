use std::{env, io};

pub enum Subcommand {
    Init,
    Run,
    Show,
    Path,
}

pub struct Options {
    // Add your options here
}

pub struct Command {
    pub subcommand: Subcommand,
    pub env: String,
    pub options: Options,
    pub args: Vec<String>,
}

pub fn parse_args() -> io::Result<Command> {
    let args: Vec<String> = env::args().collect();
    let mut env = String::from("dev");

    if args.len() < 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "No subcommand provided",
        ));
    }

    for i in 2..args.len() {
        if args[i] == "--env" {
            if i + 1 < args.len() {
                env = args[i + 1].clone();
                break;
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "No value provided for --env",
                ));
            }
        }
    }

    let subcommand = match args[1].as_str() {
        "init" => Subcommand::Init,
        "show" => Subcommand::Show,
        "run" => Subcommand::Run,
        "path" => Subcommand::Path,
        // Add more subcommands here
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid subcommand",
            ))
        }
    };

    // Parse options here

    let options = Options {
    // Set your options here
  };

    Ok(Command {
        subcommand,
        env,
        options,
        args,
    })
}
