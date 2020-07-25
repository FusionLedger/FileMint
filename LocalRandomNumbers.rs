extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    for i in 1..10 {
        println!("Random number #{}: {}", i, rng.gen_range(0, 500));
    }
}
