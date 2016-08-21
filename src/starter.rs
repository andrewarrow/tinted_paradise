use std::net::{TcpStream};
use std::io::Write;
use std::io::Read;

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
      let mut buffer = String::new();
      let _ = self.cstream.read_to_string(&mut buffer);
      println!("{:?}", buffer);
      //self.write_message(550, "No");
    }
  }

  pub fn write_message(&mut self, code: i32, message: &str) {
    let foo = format!("{} {}\r\n", code, message);
    let _ = self.cstream.write(foo.as_bytes());
  }
}

