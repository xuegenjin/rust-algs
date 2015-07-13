use rand::{thread_rng,Rng};

use std::cmp::PartialOrd;

pub fn knuth_shuffle<T:PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let r = thread_rng().gen_range(0, i + 1);
        a.swap(i, r);
    }
}
