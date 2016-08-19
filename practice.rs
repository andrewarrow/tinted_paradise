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

  hash.insert("Daniel", vec);
  println!("{:?}", hash);
}
