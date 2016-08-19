extern crate rand;
use rand::Rng;

fn main() {
  let mut vec = vec![1];
  for _ in 0..100 {
    let num = rand::thread_rng().gen_range(0, 255);
    vec.push(num)
  }
  for x in &vec {
    println!("{}", x);
  }
}
