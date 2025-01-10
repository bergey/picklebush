#![feature(str_split_remainder)]

use anyhow::anyhow;
use clap::Parser;
use regex::Regex;
use std::io;
use std::process::Command;
use std::time;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command()]
    text: Option<String>,
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

    match args.text {
        Some(t) => match_text(&regexen, &t),
        None => {
            eprintln!("enter text to match against regex patterns");
            let lines = io::stdin().lines();
            for r_text in lines {
                match_text(&regexen, &r_text.unwrap());
            }
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
        .arg("And\\(/[^\\n]*/|^ *step +\"[^\\n]*\"")
        .arg("--only-matching")
        .arg("--line-number")
        .arg(directory)
        .output()?;
    if !ripgrep.status.success() {
        eprintln!("{:?}", ripgrep);
        std::process::exit(1);
    }

    let and_re = Regex::new("/(.*)/").unwrap();
    let step_re = Regex::new("step +\"(.*)\"")?;
    let colons_re = Regex::new(":[^ \\n]+")?;

    let mut regexen = Vec::new();
    for line in std::str::from_utf8(&ripgrep.stdout)?.lines() {
        // split on : allowing : to appear after the line number
        let mut colons = line.split(':');
        let file = colons.next().unwrap_or("no : in rg output").to_string();
        let line_number = colons.next().unwrap_or("only 1 : in rg output").to_string();
        let regex_string = {
            let s = colons
                .remainder()
                .ok_or(anyhow!("nothing after second : in rg output"))?;
            if let Some('A') = s.chars().next() {
                and_re.captures(s).unwrap()[1].to_string()
            } else {
                colons_re
                    .replace_all(&step_re.captures(s).unwrap()[1], "[^ \\n]+")
                    .to_string()
            }
        };
        if let Ok(regex) = Regex::new(&regex_string) {
            regexen.push(Cucumber {
                file,
                line_number,
                regex,
            });
        } else {
            eprintln!("could not parse as regex /{regex_string}/");
        }
    }

    Ok(regexen)
}

fn match_text(regexen: &Vec<Cucumber>, text: &str) {
    for cuke in regexen {
        if cuke.regex.is_match(text) {
            println!("{}::{}", cuke.file, cuke.line_number)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn and_parse() {
        let and_re = Regex::new("/(.*)/").unwrap();
        let input = r#"And(/^\w+ do(es)? some other thing$/, () => {"#;
        let expected = r#"^\w+ do(es)? some other thing$"#.to_string();
        assert_eq!(and_re.captures(input).unwrap()[1], expected);
        let _ = Regex::new(&expected).unwrap();
    }
}
