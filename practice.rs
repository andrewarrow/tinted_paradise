extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};

fn delete_from(m: &mut HashMap<String, Vec<i32>>) {
  loop {
    println!("{}", m.len());
    let mut i = 1;
    let mut k = String::new();
    for (key, _) in m.iter() {
      println!("{} {}", i,key); 
      i += 1;
      k = key.to_string();
    }
    thread::sleep(time::Duration::from_millis(1000));
    if i > 1 {
      m.remove(&k);
    }
  }
}

fn ran_vector() -> Vec<i32> {
  let mut vec: Vec<i32> = vec![1];
  for _ in 0..100 {
    let num: i32 = rand::thread_rng().gen_range(0, 255);
    vec.push(num)
  }
  vec
}

fn ran_filename() -> String {
  let mut filename = String::new(); 
  for _ in 0..35 {
    let num = rand::thread_rng().gen_range(65, 105);
    let c = std::char::from_u32(num).unwrap();
    filename.push(c)
  }
  filename
}

fn main() {
  let mut hash: HashMap<String, Vec<i32>> = HashMap::new();
    
  for _ in 0..35 {
    hash.insert(ran_filename(), ran_vector());
  }

  let t1 = thread::spawn(move || { delete_from(&mut hash); });
  let _ = t1.join();
}
