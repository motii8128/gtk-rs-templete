use rand::{thread_rng, Rng};

fn main() {
    println!("Hello, world!");

    let mut rng = thread_rng();

    println!("{}", rng.gen_range(0..10))
}