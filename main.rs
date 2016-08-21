use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
  println!("{:?}", stream);
  let addr = stream.peer_addr().unwrap();
  println!("Got connection from {}", addr);

  let mut buf = [0; 512];
  match stream.write(b"220 Welcome to Tinted Paradise\r\n") {
            Err(_) => {},
            Ok(_) => {},
        }

  let _ = match stream.read(&mut buf) {
    Err(e) => panic!("Got an error: {}", e),
      Ok(m) => {
        if m == 0 {
          // we've got an EOF
        }
        println!("{:?}", m);
        m
      },
  };
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
