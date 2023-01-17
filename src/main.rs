#![allow(unused)]

use clap::Parser;
use std::fs;
use std::io;

mod tokenizer;
#[derive(Parser)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let mut content = String::new();

    if &args.pattern == "parse" {
        let content = fs::read_to_string(&args.path).expect("could not read file");
        let content_chars = content.chars();

        for c in content_chars {
            // tokenizer::token(c);
        }
    }
}
