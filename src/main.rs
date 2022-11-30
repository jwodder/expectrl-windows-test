use expectrl::{spawn, Error};

fn main() -> Result<(), Error> {
    let mut p = spawn("cat")?;
    p.send_line("Hello World")?;
    p.expect("Hello World")?;
    Ok(())
}
