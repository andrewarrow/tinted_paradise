extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::{Arc, Mutex};

fn delete_from(m: &Mutex<HashMap<String, Vec<i32>>>) {
  loop {
    println!("{}", m.lock().unwrap().len());

    let mut i = 1;
    let mut k = String::new();
    for (key, _) in m.lock().unwrap().iter() {
      println!("{} {}", i, key); 
      i += 1;
      k = key.to_string();
    }
    if i > 1 {
      m.lock().unwrap().remove(&k);
    }

    thread::sleep(time::Duration::from_millis(1000));
  }
}

fn insert_into(m: &Mutex<HashMap<String, Vec<i32>>>) {
  let mut i = 1;
  loop {
    let num: u64 = rand::thread_rng().gen_range(10, 800);
    thread::sleep(time::Duration::from_millis(num));
    m.lock().unwrap().insert(ran_filename(), ran_vector());
    i += 1;
    if i > 10 {
      thread::sleep(time::Duration::from_millis(20000));
      i = 1
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
  let hash: Arc<Mutex<HashMap<String, Vec<i32>>>> = Arc::new(Mutex::new(HashMap::new()));
  let clone1 = hash.clone();
  let clone2 = hash.clone();
  let t1 = thread::spawn(move || { delete_from(&clone1); });
  thread::spawn(move || { insert_into(&clone2); });
  let _ = t1.join();
}
