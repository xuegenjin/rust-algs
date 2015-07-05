extern crate algs4;

use algs4::union_find::{UF, QuickFind, QuickUnion, WeightedQuickUnion};
//use algs4::percolation::PercolationStats;

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

    /*if env::args().count() > 2 {
        let args = env::args().skip(1).take(2).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let ps = PercolationStats::new(args[0], args[1]);

        println!("mean                    = {}", ps.mean());
        println!("stddev                  = {}", ps.stddev());
        println!("95% confidence interval = {}, {}" , ps.confidence_lo(), ps.confidence_hi());
    }*/
}
