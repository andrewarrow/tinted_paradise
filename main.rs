use std::net::{TcpListener,TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
  let addr = stream.peer_addr().unwrap();
  println!("Got connection from {}", addr);
}

fn main() { 
  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
      thread::spawn(|| {
          handle_client(stream.unwrap());
      });
  }
}
