Loaded Dice
============

A simple crate that implements a random sampler implementing the alias method (https://en.wikipedia.org/wiki/Alias_method). It can be used to sample from discrete probability distributions efficiently (`O(1)` per sample). One uses it by passing a vector of probabilites to the constructor. The constructor builds a data structure in `O(n*n*log(n))` (Note: It would be quite possible to implement this in `O(n*log(n))`, however for reasonable sized number of values this method is faster than using the more effictient data strcutres. If the construction is slow in your case, you might consider using min/max heaps insteat of resorting the array after each construction step). This data structure can then be used to to sample a numbers between `0` and `n` with the corresponding probabilites.

Assume we want to sample from the following distirbution: `p(0)=0.5, p(1)=0.3, p(2)=0.1, p(3)=0.1`:

```rust
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
```
