extern crate rand;
extern crate sys_info;
use rand::Rng;
use std::collections::HashMap;
use std::{thread, time};
use std::sync::{Arc, Mutex};
use sys_info::*;

fn delete_from(m: &Mutex<HashMap<String, Vec<i32>>>) {
  loop {
    println!("{}", m.lock().unwrap().len());

    let mut i = 1;
    let mut k = String::new();
    for (key, v) in m.lock().unwrap().iter() {
      println!("{} {} {}", i, key, v.len()); 
      i += 1;
      if k.len() == 0 {
        k = key.to_string();
      }
    }
    if i > 1 {
      //m.lock().unwrap().remove(&k);
    }

    let mem = mem_info().unwrap();
    println!("swap: total {} KB, free {} KB", mem.swap_total, mem.swap_free);
    println!("proc total: {}", proc_total().unwrap());
    println!("mem: total {} KB, free {} KB, avail {} KB, buffers {} KB, cached {} KB",
        mem.total, mem.free, mem.avail, mem.buffers, mem.cached);
    println!("mem: total {} MB, free {} MB, avail {} MB, buffers {} MB, cached {} MB",
        mem.total/1000, mem.free/1000, mem.avail/1000, mem.buffers/1000, mem.cached/1000);
    thread::sleep(time::Duration::from_millis(1000));
  }
}

fn insert_into(m: &Mutex<HashMap<String, Vec<i32>>>) {
  loop {
    let num: u64 = rand::thread_rng().gen_range(10, 900);
    thread::sleep(time::Duration::from_millis(num));
    m.lock().unwrap().insert(ran_filename(), ran_vector());
    if m.lock().unwrap().len() > 100 {
      while m.lock().unwrap().len() > 0 {
        thread::sleep(time::Duration::from_millis(100));
      }
    }
  }
}

fn ran_vector() -> Vec<i32> {
  let mut vec: Vec<i32> = vec![1];
  let s = rand::thread_rng().gen_range(100, 1000000);
  for _ in 0..s {
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
