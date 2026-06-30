mod cli;
mod common;
mod lexer;
mod parser;
mod sema;
mod mir;

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::ColorSpec;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Chars, Config, Styles};
use colored::Colorize;
use common::Context;
use std::path::PathBuf;
use std::{env, fs};

const STATUS: &str = "Pre-Alpha";

fn print_version() {
    println!("Miel Language Compiler, Version {} {}", env!("CARGO_PKG_VERSION"), STATUS);
    println!("Licensed under the Apache License, Version 2.0");
}

fn print_help() {
    print_version();
    println!();
    println!("Usage: miel [OPTIONS] <INPUT>");
    println!();
    println!("Options:");
    println!("  -h, --help                Print this help message");
    println!("  -V, --version             Print version");
    println!("  -o, --output <FILE>       Write output to FILE");
    println!("  -c, --no-color            Disable colored output");
}

fn main() {
    let command = match cli::parse_command(env::args()) {
        Ok(o) => o,
        Err(err) => match &*err {
            "print help" => {
                print_help();
                return;
            },
            "print version" => {
                print_version();
                return;
            },
            other => {
                eprintln!("{}: {other}", "error".red().bold().underline());
                return;
            },
        }
    };
    colored::control::set_override(!command.no_color);

    let contents = match fs::read_to_string(&command.input) {
        Ok(o) => o,
        Err(err) => {
            eprintln!("{}: {err}", "error".red().bold().underline());
            return;
        }
    };

    let mut files = SimpleFiles::new();
    let file_id = files.add(&command.input, &contents);
    let config = {
        let mut styles = Styles::default();
        {
            let mut cspec = ColorSpec::new();
            cspec
                .set_fg(Some(term::termcolor::Color::Red))
                .set_bold(true)
                .set_underline(true);
            styles.header_error = cspec;
        }
        styles.header_message = ColorSpec::new();
        let chars = Chars {
            source_border_top_left: ',',
            source_border_top: '-',
            source_border_left: '|',
            source_border_left_break: ':',
            note_bullet: '*',
            single_primary_caret: '^',
            single_secondary_caret: '~',
            multi_primary_caret_start: '^',
            multi_primary_caret_end: '^',
            multi_secondary_caret_start: '\'',
            multi_secondary_caret_end: '\'',
            multi_top_left: ',',
            multi_top: '-',
            multi_bottom_left: '\'',
            multi_bottom: '-',
            multi_left: '|',
            pointer_left: '|',
        };
        Config {
            styles,
            chars,
            ..Config::default()
        }
    };

    let mut writer = StandardStream::stderr(if command.no_color {
        ColorChoice::Never
    } else {
        ColorChoice::Always
    });

    let mut rodeo = lasso::Rodeo::new();

    let tokens = match lexer::tokenize(file_id, &contents, &mut rodeo) {
        Ok(o) => o,
        Err(err) => {
            term::emit(&mut writer, &config, &files, &err)
                .expect("Diagnostic formatting failed");
            return;
        }
    };
    
    let ctx = Context {
        rodeo: &rodeo,
        source_id: file_id,
    };
    let ast = match parser::Parser::new(&ctx, &tokens).parse() {
        Ok(o) => o,
        Err(err) => {
            term::emit(&mut writer, &config, &files, &err)
                .expect("Diagnostic formatting failed");
            return;
        }
    };

    let module_name = PathBuf::from(&command.input).file_stem()
        .unwrap()
        .display()
        .to_string();
        
    let mut codegen = mir::codegen::Codegen::new(&module_name, ctx);
    let module = match codegen.generate(&ast) {
        Ok(()) => codegen.module,
        Err(err) => {
            term::emit(&mut writer, &config, &files, &err)
                .expect("Diagnostic formatting failed");
            return;
        },
    };
    eprintln!("{module:?}");
}
