use std::process::Command;

fn main() -> std::io::Result<()> {
    let output = Command::new("echo").arg("Hello world").output()?;
    println!("{:?}", output);
    Ok(())
}
