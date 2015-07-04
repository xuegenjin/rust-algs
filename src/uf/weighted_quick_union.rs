use super::union_find::UF;

pub struct WeightedQuickUnion {
    id: Vec<usize>,
    sz: Vec<usize>,
    count: usize
}

impl WeightedQuickUnion {
    pub fn new(n: usize) -> WeightedQuickUnion {
        WeightedQuickUnion {
            id: (0..n).collect(),
            sz: vec![1; n],
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

impl UF for WeightedQuickUnion {
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

        if self.sz[i] < self.sz[j] {
            self.id[i] = j;
            self.sz[j] += self.sz[i];
        } else {
            self.id[j] = i;
            self.sz[i] += self.sz[j];
        }

        self.count -= 1;
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_tiny() {
    let mut uf = WeightedQuickUnion::new(10);

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
}

#[test]
fn test_worst_case() {
    let mut uf = WeightedQuickUnion::new(5);

    uf.union(0,1);
    uf.union(0,2);
    uf.union(0,3);
    uf.union(0,4);

    assert_eq!(1, uf.max_path_length());
}

#[test]
fn test_worst_case2() {
    let mut uf = WeightedQuickUnion::new(8);

    uf.union(0,1);
    uf.union(2,3);
    uf.union(4,5);
    uf.union(6,7);
    uf.union(0,2);
    uf.union(4,6);
    uf.union(0,4);

    assert_eq!(3, uf.max_path_length());
}
