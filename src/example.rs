#[cfg(feature = "std")]
extern crate alloc;
#[cfg(feature = "std")]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use loaded_dice::LoadedDiceSampler;
#[cfg(feature = "std")]
use rand::thread_rng;

#[cfg(feature = "std")]
fn main() {
    let mut s = LoadedDiceSampler::new(vec![0.5, 0.3, 0.1, 0.1], thread_rng());
    let iter: usize = 100;
    for _ in 0..iter {
        println!("{}", s.sample());
    }
}

#[cfg(not(feature = "std"))]
fn main() {
    panic!("Run with std feature");
}
