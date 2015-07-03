extern crate algs4;

use algs4::uf::union_find::UF;
use algs4::uf::quick_find::QuickFind;

fn main() {
    let mut qf = QuickFind::new(10);
    //println!("{:?}", &qf.id);
    qf.union(4,3);
    assert!(qf.connected(4,3));
}
