
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
  }

  pub fn writeMessage(&mut self, code: i32, message: &str) {
    let buffer = String::new();
    self.cstream.write(b"220 Welcome to Tinted Paradise\r\n");
  }
}


/*
fn handle_client(mut stream: TcpStream) {
  let mut br = BufReader::new(&stream);

  loop {

    let mut buffer = String::new();
    let _ = br.read_line(&mut buffer);
    println!("{:?}", buffer);
  }
}
*/
