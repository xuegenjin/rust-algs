extern crate algs4;

use algs4::union_find::{UF, QuickFind, QuickUnion, WeightedQuickUnion};
//use algs4::percolation::PercolationStats;
use algs4::sorting::*;

use std::env;

fn main() {
    let mut qf = QuickFind::new(10);
    qf.union(4,3);
    assert!(qf.connected(4,3));

    let mut qu = QuickUnion::new(10);
    qu.union(4,3);
    assert!(qu.connected(4,3));

    let mut wqu = WeightedQuickUnion::new(10);
    wqu.union(4,3);
    assert!(wqu.connected(4,3));

    let mut v = vec![5, 4, 3, 2, 1];
    selection_sort(&mut v);
    println!("{:?}", v);
}
