use super::union_find::UF;

pub struct QuickFind {
    id: Vec<usize>,
    count: usize
}

impl QuickFind {
    pub fn new(n : usize) -> QuickFind {
        QuickFind { id : (0..n).collect(), count: n}
    }
}

impl UF for QuickFind {
    fn union (&mut self, p:usize, q: usize) {
        let pid = self.find(p);
        let qid = self.find(q);

        if pid != qid {
            for i in 0..self.id.len() {
                if self.id[i] == pid {
                    self.id[i] = qid;
                }
            }

            self.count -= 1;
        }
    }

    fn find(&self, p:usize) -> usize {
        self.id[p]
    }

    fn count(&self) -> usize {
        self.count
    }
}

#[test]
fn test_tiny() {
    let mut qf = QuickFind::new(10);

    qf.union(4,3);
    qf.union(3,8);
    qf.union(6,5);
    qf.union(9,4);
    qf.union(2,1);

    assert!(qf.connected(8,9));

    qf.union(5,0);
    qf.union(7,2);
    qf.union(6,1);

    assert!(qf.connected(1,0));
    assert!(qf.connected(6,7));

    assert_eq!(2, qf.count());
}
