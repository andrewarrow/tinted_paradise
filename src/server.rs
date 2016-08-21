use std::net::{TcpStream};
use std::io::Write;
use std::io::Read;
use std::str;

use auth;

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
      let chars = self.cstream.read(&mut buffer).unwrap()-2;
      println!("{}", chars);
      let line = str::from_utf8(&buffer[0..chars]).unwrap();
      let v: Vec<&str> = line.split_terminator(' ').collect();
      println!("{:?}", v);
      let command = v[0];
      let param = v[1];
      //self.write_message(331, "User name ok, password required");
      auth::handle_user();
    }
  }

  pub fn write_message(&mut self, code: i32, message: &str) {
    let foo = format!("{} {}\r\n", code, message);
    let _ = self.cstream.write(foo.as_bytes());
  }
}

