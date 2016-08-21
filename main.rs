use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::io::Read;
use std::io::Write;
use std::str;
use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn handle_client(mut stream: TcpStream) {
  println!("{:?}", stream);
  let addr = stream.peer_addr().unwrap();
  println!("Got connection from {}", addr);

  match stream.write(b"220 Welcome to Tinted Paradise\r\n") {
            Err(_) => {},
            Ok(_) => {},
        }

  let mut br = BufReader::new(&stream);

  loop {

    let mut buffer = String::new();
    br.read_line(&mut buffer);
    println!("{:?}", buffer);
  }
}

fn main() { 
  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
    match stream {
      Err(e) => { println!("failed: {}", e) }
      Ok(stream) => {
        thread::spawn(move || {
            handle_client(stream)
            });
      }
    }
  }
}
