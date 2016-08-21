use std::net::{TcpStream};
use std::io::Write;
use std::io::Read;
use std::str;

#[derive(Debug)]
pub struct Paradise {
  cstream: TcpStream
}

impl Paradise {
  pub fn new(stream: TcpStream) -> Paradise {
    Paradise {cstream: stream}
  }

  pub fn start(&mut self) {
    self.write_message(220, "Welcome to Paradise");
    loop {
      let mut buffer = [0; 100];
      let chars = self.cstream.read(&mut buffer).unwrap();
      println!("{}", chars);
      let heart = str::from_utf8(&buffer).unwrap();
      println!("|{}|", heart.trim());
      //self.write_message(331, "User name ok, password required");
    }
  }

  pub fn write_message(&mut self, code: i32, message: &str) {
    let foo = format!("{} {}\r\n", code, message);
    let _ = self.cstream.write(foo.as_bytes());
  }
}

