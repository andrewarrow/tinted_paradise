extern crate paradise;

use std::net::TcpListener;
use std::thread;
use std::collections::HashMap;
use paradise::server::Paradise;
use paradise::auth;

fn main() { 
  let mut command_map: HashMap<String, fn()> = HashMap::new();
  command_map.insert("USER".to_string(), auth::handle_user);
  println!("{:?}", command_map);

  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
    match stream {
      Err(e) => { println!("failed: {}", e) }
      Ok(stream) => {
        let addr = stream.peer_addr().unwrap();
        println!("Got connection from {}", addr);

        let mut p = Paradise::new(stream, &command_map);
        println!("{:?}", p);
        thread::spawn(move || {
          p.start();
        });
      }
    }
  }
}
