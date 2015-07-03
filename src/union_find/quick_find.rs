use super::uf::UF;

pub struct QuickFind {
    id: Vec<usize>,
    count: usize
}

impl QuickFind {
    pub fn new(n : usize) -> QuickFind {
        QuickFind {
            id : (0..n).collect(),
            count: n
        }
    }
}

impl UF for QuickFind {
    fn find(&self, p:usize) -> usize {
        self.id[p]
    }

    fn union (&mut self, p:usize, q: usize) {
        let pid = self.find(p);
        let qid = self.find(q);

        if pid == qid {
            return;
        }

        for i in 0..self.id.len() {
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }

        self.count -= 1;
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_tiny() {
    let mut uf = QuickFind::new(10);

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
