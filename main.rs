use std::net::{TcpListener,TcpStream};
use std::thread;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::io::Read;
use std::io::Write;
use std::str;

fn handle_client(mut stream: TcpStream) {
  println!("{:?}", stream);
  let addr = stream.peer_addr().unwrap();
  println!("Got connection from {}", addr);

  match stream.write(b"220 Welcome to Tinted Paradise\r\n") {
            Err(_) => {},
            Ok(_) => {},
        }

  loop {
    let mut buf = [0; 512];
    let _ = match stream.read(&mut buf) {
      Err(e) => panic!("Got an error: {}", e),
        Ok(m) => {
          if m == 0 {
            break;
            // we've got an EOF
          }
          let heart = str::from_utf8(&buf).unwrap();
          println!("{:?}", heart);
          m
        },
    };
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
