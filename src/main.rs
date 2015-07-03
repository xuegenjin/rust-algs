extern crate algs4;

use algs4::uf::union_find::UF;
use algs4::uf::quick_find::QuickFind;
use algs4::uf::quick_union::QuickUnion;

fn main() {
    let mut qf = QuickFind::new(10);
    qf.union(4,3);
    assert!(qf.connected(4,3));

    let mut qu = QuickUnion::new(10);
    qu.union(4,3);
    assert!(qu.connected(4,3));
}
