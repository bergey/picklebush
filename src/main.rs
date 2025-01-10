use std::process::Command;

fn main() -> anyhow::Result<()> {
    let directory = "/Users/bergey/braze/platform/develop/dashboard";
    let regexen = Command::new("rg")
        .arg("And\\(/([^\\n]*)/")
        .arg("--only-matching")
        .arg("--line-number")
        .arg("--replace")
        .arg("$1")
        .arg(directory)
        .output()?;

    if !regexen.status.success() {
        eprintln!("{:?}", regexen);
        std::process::exit(1);
    }
    let mut counter: u64 = 0;
    for line in std::str::from_utf8(&regexen.stdout)?.lines() {
        counter += 1;
    }
    println!("{counter}");

    Ok(())
}
