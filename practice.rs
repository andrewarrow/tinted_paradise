extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::thread;

fn count(m: &mut HashMap<String, Vec<i32>>) {
  loop {
    println!("{}", m.len());
  }
}

fn main() {
  let mut hash: HashMap<String, Vec<i32>> = HashMap::new();
  let mut vec: Vec<i32> = vec![1];
  for _ in 0..100 {
    let num: i32 = rand::thread_rng().gen_range(0, 255);
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

  //println!("{:?}", hash);
  thread::spawn(move || { count(&mut hash); });
  loop {
    println!("looping");
  }
}
