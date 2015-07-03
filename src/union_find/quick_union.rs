use super::uf::UF;

pub struct QuickUnion {
    id: Vec<usize>,
    count: usize
}

impl QuickUnion {
    pub fn new(n: usize) -> QuickUnion {
        QuickUnion {
                id: (0..n).collect(),
                count: n
        }
    }

    fn path_length(&self, i: usize, length: usize) -> usize {
        if i == self.id[i] {
            length
        } else {
            self.path_length(self.id[i], length + 1)
        }
    }

    fn max_path_length(&self) -> usize {
        (0..self.id.len())
            .map (|i| self.path_length(i, 0))
            .max()
            .unwrap()
    }
}

impl UF for QuickUnion {
    fn find(&self, p:usize) -> usize {
        let mut p = p;
        while p != self.id[p] {
            p = self.id[p];
        }
        p
    }

    fn union (&mut self, p:usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);

        if i == j {
            return;
        }

        self.id[i] = j;
        self.count -= 1;
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_tiny() {
    let mut uf = QuickUnion::new(10);

    uf.union(4,3);
    uf.union(3,8);
    uf.union(6,5);
    uf.union(9,4);
    uf.union(2,1);

    assert!(uf.connected(8,9));

    uf.union(5,0);
    uf.union(7,2);
    uf.union(6,1);

    assert!(uf.connected(1,0));
    assert!(uf.connected(6,7));

    assert_eq!(2, uf.count());

    assert_eq!(3, uf.max_path_length());
}

#[test]
fn test_worst_case() {
    let mut uf = QuickUnion::new(5);

    uf.union(0,1);
    uf.union(0,2);
    uf.union(0,3);
    uf.union(0,4);

    assert_eq!(4, uf.max_path_length());

}
