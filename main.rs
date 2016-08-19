use std::net::{TcpListener,TcpStream};
use std::io::{BufRead,BufReader};
use std::thread;
use std::sync::mpsc::{channel, Receiver, TryRecvError};

fn handle_client(stream: TcpStream) {
}

fn main() { 
  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
      let (ds, _) = channel();
      thread::spawn(|| {
        let addr = stream(TcpStream).peer_addr().unwrap();
        println!("Got connection from {}", addr);

        let mut reader = BufReader::new(stream);
        let mut name_buf = Vec::new();

        match reader.read_until(0, &mut name_buf) {
          Ok(_) => {
            let foo = String::from_utf8(name_buf).unwrap();
            println!("foo {}", foo);
          },
            Err(_) => {
              drop(ds);
              println!("error");
              return;
            },
        }

        loop {
          let result = match reader.fill_buf() {
            Ok(data) if data.len() == 0 => Some(0),
              Ok(data) => { ds.send(Ok(data.to_vec())).unwrap(); Some(data.len()) },
              Err(e) => { ds.send(Err(e)).unwrap(); None },
          };

          if let Some(read) = result {
            if read > 0 {
              reader.consume(read);
            } else {
              drop(ds);
              break;
            }
          }
        }
      });
  }
}
