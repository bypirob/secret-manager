mod cli;
mod dotenv;
use std::{env, ffi::CString, io, path};

macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            dbg!($($arg)*);
        }
        #[cfg(not(debug_assertions))]
        {
            // Do nothing
        }
    };
}

fn get_env_file(env: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    let project = current_dir.to_str().unwrap().split("/").last().unwrap();
    let home_dir = env::var("HOME").unwrap();
    let filename = format!("{home_dir}/.secret-manager/{project}-{env}.env");
    debug_print!(&filename);
    filename
}

fn run_with_env(command: cli::Command) -> io::Result<()> {
    let filename = get_env_file(&command.env);

    let value = dotenv::read_env_file(&filename)?;
    // dbg!(&value);

    for (k, v) in &value {
        env::set_var(k, v.to_string());
    }

    let cmd_args: Vec<CString> = command.args[2..command.args.len()]
        .iter()
        .map(|arg| CString::new(arg.as_str()).expect("CString::new failed"))
        .collect();

    let mut argv: Vec<*const libc::c_char> = cmd_args.iter().map(|arg| arg.as_ptr()).collect();
    argv.push(std::ptr::null());

    // dbg!(&cmd_args);

    unsafe {
        let r = libc::execvp(argv[0], argv.as_ptr());
        dbg!(r);
        dbg!(std::io::Error::last_os_error());
    };

    Ok(())
}

fn init_env(command: cli::Command) -> io::Result<()> {
    let filename = get_env_file(&command.env);

    if !path::Path::new(&filename).exists() {
        dotenv::create_env_file(&filename)?;
    } else {
        println!("File already exists: {}", filename);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let command = cli::parse_args()?;

    match command.subcommand {
        cli::Subcommand::Init => {
            init_env(command)?;
        }
        cli::Subcommand::Run => {
            run_with_env(command)?;
        }
        cli::Subcommand::Show => {
            let filename = get_env_file(&command.env);
            let value = dotenv::read_env_file(&filename)?;
            dbg!(&value);
        }
        cli::Subcommand::Path => {
            let filename = get_env_file(&command.env);
            println!("{}", filename);
        }
    }

    return Ok(());
}
