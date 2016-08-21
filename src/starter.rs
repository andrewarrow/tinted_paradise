use std::io::BufReader;
use std::io::BufRead;
use std::net::{TcpStream};
use std::io::Write;

#[derive(Debug)]
pub struct Paradise {
  cstream: TcpStream
}

impl Paradise {
  pub fn new(stream: TcpStream) -> Paradise {
    Paradise {cstream: stream}
  }

  pub fn start(&mut self) {
    self.writeMessage(220, "Welcome to Paradise");
    let mut br = BufReader::new(&self.cstream);
    loop {
      let mut buffer = String::new();
      let _ = br.read_line(&mut buffer);
      println!("{:?}", buffer);
    }
  }

  pub fn writeMessage(&mut self, code: i32, message: &str) {
    let mut buffer = String::new();
    buffer.push_str("220 ");
    buffer.push_str(message);
    buffer.push_str("\r\n");
    self.cstream.write(buffer.as_bytes());
  }
}

