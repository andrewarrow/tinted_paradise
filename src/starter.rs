
use std::net::{TcpStream};

#[derive(Debug)]
pub struct Paradise {
  cstream: TcpStream
}

impl Paradise {
  pub fn new(mut stream: TcpStream) -> Paradise {
    Paradise {cstream: stream}
  }
}
