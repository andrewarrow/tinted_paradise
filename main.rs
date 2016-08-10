use std::net::{TcpListener,TcpStream};
use std::io::{BufRead,BufReader,Read};
use std::thread;

fn handle_client(stream: TcpStream) {
  let addr = stream.peer_addr().unwrap();
  println!("Got connection from {}", addr);

  //let mut writer = stream.try_clone().unwrap();
  let mut reader = BufReader::new(stream);
  let mut data: Vec<u8> = Vec::new();

  let mut file_name_buf = String::new();
  reader.read_line(&mut file_name_buf).unwrap();

  reader.read_to_end(&mut data).unwrap();

  println!("gots the datas...{}",data.len());
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
