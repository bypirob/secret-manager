use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
};

type Table = std::collections::HashMap<String, String>;

pub fn create_env_file(file_path: &str) -> io::Result<()> {
    let parent_dir = std::path::Path::new(file_path).parent().unwrap();
    fs::create_dir_all(parent_dir)?;

    let mut file = fs::File::create(file_path)?;
    file.write_all(b"KEY=VALUE\n")?;

    return Ok(());
}

pub fn read_env_file(file_path: &str) -> io::Result<Table> {
    let value: Table = match parse_env_file(file_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to load dotenv file: {}", e);
            std::process::exit(1);
        }
    };

    for (k, v) in &value {
        env::set_var(k, v.to_string());
    }

    return Ok(value);
}

pub fn parse_env_file<S: Into<String>>(path: S) -> io::Result<Table> {
    let file = fs::read_to_string(path.into())?;
    let mut data = HashMap::new();

    file.split("\n").for_each(|v| {
        if v == "" {
            return;
        }

        if let Some((k, v)) = v.split_once("=") {
            let v_parsed = strip_quotes(v);
            data.insert(k.into(), v_parsed.into());
        }
    });

    return Ok(data);
}

fn strip_quotes(s: &str) -> &str {
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}
