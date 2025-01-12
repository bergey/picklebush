#![feature(str_split_remainder)]

use anyhow::anyhow;
use clap::Parser;
use regex::Regex;
use std::process::Command;
use std::time;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command()]
    text: String,
    #[arg(long = "debug")]
    debug: bool,
    #[arg(long = "dir", short = 'd')]
    directory: Option<String>,
}

struct Cucumber {
    file: String,
    line_number: String, // no reason to parse & then reassemble
    regex: Regex,
}

fn main() -> anyhow::Result<()> {
    let start = time::Instant::now();
    let args = Args::parse();

    let regexen = load_regexen(args.directory.as_deref().unwrap_or("."))?;
    let parsing_t = start.elapsed();

    for cuke in &regexen {
        if cuke.regex.is_match(&args.text) {
            println!("{}::{}", cuke.file, cuke.line_number)
        }
    }
    let total_t = start.elapsed();

    if args.debug {
        eprintln!("found {} matching regexen", regexen.len());
        eprintln!(
            "load & parsing {}ms total {}ms",
            parsing_t.as_millis(),
            total_t.as_millis(),
        );
    }
    Ok(())
}

fn load_regexen(directory: &str) -> anyhow::Result<Vec<Cucumber>> {
        let ripgrep = Command::new("rg")
        .arg("And\\(/([^\\n]*)/")
        .arg("--only-matching")
        .arg("--line-number")
        .arg("--replace")
        .arg("$1")
        .arg(directory)
        .output()?;
    if !ripgrep.status.success() {
        eprintln!("{:?}", ripgrep);
        std::process::exit(1);
    }

    let mut regexen = Vec::new();
    for line in std::str::from_utf8(&ripgrep.stdout)?.lines() {
        // split on : allowing : to appear after the line number
        let mut colons = line.split(':');
        let file = colons.next().unwrap_or("no : in rg output").to_string();
        let line_number = colons.next().unwrap_or("only 1 : in rg output").to_string();
        let regex = Regex::new(
            colons
                .remainder()
                .ok_or(anyhow!("nothing after second : in rg output"))?,
        )?;
        regexen.push(Cucumber {
            file,
            line_number,
            regex,
        });
    }

    Ok(regexen)
}
