//! CLI Args Parsing

use std::env::Args;

#[allow(unused)]
pub struct Command {
    pub input: String,
    pub output: Option<String>,
    pub no_color: bool,
}

pub fn parse_command(mut args: Args) -> Result<Command, String> {
    args.next();
    let mut input = None;
    let mut output = None;
    let mut no_color = false;
    while let Some(arg) = args.next() {
        match &*arg {
            "--output" | "-o" => match (args.next(), output) {
                (Some(a), None) => output = Some(a),
                (_, Some(_)) => return Err("Output path already defined".to_string()),
                (None, _) => return Err(format!("Expected output path after `{arg}` argument")),
            },
            "--no-color" => no_color = true,
            other if other.starts_with("-") => {
                for ch in other[1..].chars() {
                    match ch {
                        'c' => no_color = true,
                        'h' => return Err("print help".to_string()),
                        'V' => return Err("print version".to_string()),
                        _ => continue,
                    }
                }
            },
            "--help" => return Err("print help".to_string()),
            "--version" => return Err("print version".to_string()),
            _ => match input {
                None => input = Some(arg),
                Some(_) => return Err("Input path already defined".to_string()),
            },
        }
    }
    if input.is_none() {
        return Err("Expected input path".to_string());
    }
    Ok(Command {
        input: input.unwrap(),
        output,
        no_color,
    })
}
