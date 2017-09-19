extern crate loaded_dice;
extern crate rand;

use loaded_dice::LoadedDiceSampler;
use rand::{thread_rng, Rng};
fn main() {
    let mut s = LoadedDiceSampler::new(vec!(0.5, 0.3, 0.1, 0.1), thread_rng());
    let iter: usize = 100;
    for i in (0..iter) {
        println!("{}", s.sample());
    }
}
