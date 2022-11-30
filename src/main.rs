use expectrl::{spawn, Error};
use std::time::Duration;

fn main() -> Result<(), Error> {
    println!("Spawning ...");
    let mut p = spawn("cat")?;
    p.set_expect_timeout(Some(Duration::from_secs(3)));
    println!("Sending line ...");
    p.send_line("Hello World")?;
    println!("Expecting line ...");
    p.expect("Hello World")?;
    println!("OK!");
    Ok(())
}
