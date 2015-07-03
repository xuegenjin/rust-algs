extern crate algs4;

use algs4::union_find::uf::UF;
use algs4::union_find::quick_find::QuickFind;
use algs4::union_find::quick_union::QuickUnion;

fn main() {
    let mut qf = QuickFind::new(10);
    qf.union(4,3);
    assert!(qf.connected(4,3));

    let mut qu = QuickUnion::new(10);
    qu.union(4,3);
    assert!(qu.connected(4,3));
}
