use std::net::TcpListener;

pub use self::starter::Paradise;
mod starter;

fn main() { 
  let listener = TcpListener::bind("127.0.0.1:2121").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
    match stream {
      Err(e) => { println!("failed: {}", e) }
      Ok(stream) => {
        let addr = stream.peer_addr().unwrap();
        println!("Got connection from {}", addr);

        let mut p = Paradise::new(stream);
        println!("{:?}", p);
        p.start();
      }
    }
  }
}
