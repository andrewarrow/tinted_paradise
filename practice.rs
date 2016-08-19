extern crate rand;
use rand::Rng;
use std::collections::HashMap;

fn main() {
  let mut hash = HashMap::new();
  let mut vec = vec![1];
  for _ in 0..100 {
    let num = rand::thread_rng().gen_range(0, 255);
    vec.push(num)
  }
  println!("{}", vec.len());
    
  for _ in 0..35 {
    let mut filename = String::new(); 
    for _ in 0..35 {
      let num = rand::thread_rng().gen_range(65, 105);
      let c = std::char::from_u32(num).unwrap();
      filename.push(c)
    }
    hash.insert(filename, vec.clone());
  }

  println!("{:?}", hash);
}
