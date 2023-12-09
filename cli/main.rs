use lib::util::RW;
use std::io::{Read, Write};

struct StdBuf;

impl RW for StdBuf {
    fn read(&mut self) -> Option<u8> {
        std::io::stdin().bytes().next().and_then(|r| r.ok())
    }
    fn write(&mut self, b: u8) {
        std::io::stdout().write(&[b]).unwrap();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    std::io::stdout().write(b"prog> ")?;
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut buf)?; // read program from user input
    let mut io = StdBuf;
    let mut machine = lib::bf::Machine::new(lib::util::Config::default());
    machine.exec(buf.into(), &mut io)?;
    Ok(())
}
