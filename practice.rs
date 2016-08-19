extern crate rand;
use rand::Rng;

fn main() {
  // a number from [-40.0, 13000.0)
  let num = rand::thread_rng().gen_range(0, 255);
  println!("{}", num);
}
