#![no_std]

#[macro_use]
extern crate alloc;
use alloc::vec::Vec;
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
struct AliasEntry {
    val: usize,
    alias: usize,
    prob_of_val: f64,
}

impl AliasEntry {
    pub fn new(val: usize, alias: usize, prob_of_val: f64) -> Self {
        AliasEntry {
            val,
            alias,
            prob_of_val,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LoadedDiceSampler<R: Rng> {
    entries: Vec<AliasEntry>,
    rng: R,
}

impl<R: Rng> LoadedDiceSampler<R> {
    pub fn new(probs: Vec<f64>, rng: R) -> Self {
        let entries = LoadedDiceSampler::<R>::construct_table(probs);
        Self { entries, rng }
    }

    pub fn sample(&mut self) -> usize {
        let index = self.rng.gen_range(0..self.entries.len());
        let coin = self.rng.gen::<f64>();
        let entry = &self.entries[index];
        if coin > entry.prob_of_val {
            entry.alias
        } else {
            entry.val
        }
    }

    fn construct_table(probs: Vec<f64>) -> Vec<AliasEntry> {
        let mut res = vec![];
        let n = probs.len() as f64;
        let inv_n = 1.0 / probs.len() as f64;

        let mut tmp = { probs.into_iter().enumerate().collect::<Vec<_>>() };

        while tmp.len() > 1 {
            //rust sort ist optimized for nearly sorted cases, so I assume that a
            //better implementation with priority queues might actually be slower, however if you
            //run into performance troubles, replace tmp with a min/max heap
            tmp.sort_by(|&(_, p1), &(_, p2)| p2.partial_cmp(&p1).unwrap()); // [biggest-prob, ..., smallest-prob]
            let (min_i, min_p) = tmp.pop().unwrap();
            let &mut (ref max_i, ref mut max_p) = tmp.get_mut(0).unwrap();
            res.push(AliasEntry::new(min_i, *max_i, min_p * n));
            let used_prob = inv_n - min_p;
            *max_p -= used_prob;
        }
        let (last_i, last_p) = tmp.pop().unwrap();
        assert!(0.999 < last_p * n && last_p * n < 1.001); // last value should always be exactly 1 but floats...
        res.push(AliasEntry::new(last_i, core::usize::MAX, 1.0));

        res
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use crate::LoadedDiceSampler;
    use alloc::vec::Vec;
    use rand::{thread_rng, Rng};

    #[test]
    fn it_works() {
        let mut rng = thread_rng();
        let len = rng.gen_range(3..10);
        let base = (0..len).map(|_| rng.gen::<f64>()).collect::<Vec<_>>();
        let sum: f64 = base.iter().sum();
        let base = base.iter().map(|v| v / sum).collect::<Vec<_>>();
        let mut s = LoadedDiceSampler::new(base.clone(), rng);
        let mut res: Vec<usize> = vec![0; len];
        let iter: usize = 1000000;
        for _ in 0..iter {
            let i = s.sample();
            res[i] += 1;
        }
        let _res_p = res
            .iter()
            .map(|&f| f as f64 / iter as f64)
            .collect::<Vec<_>>();
        //println!("{:?}", res_p);
        for (i, c) in res.iter().enumerate() {
            let p_i = *c as f64 / iter as f64;
            assert!(base[i] * 0.99 < p_i && base[i] * 1.01 > p_i);
        }
    }
}
