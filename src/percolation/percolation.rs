use ::union_find::{UF, WeightedQuickUnion};

pub struct Percolation {
    n: usize,
    num_sites: usize,
    sites: Vec<bool>,
    uf_with_top_bottom: WeightedQuickUnion, //UF,
    uf_with_top_only: WeightedQuickUnion //UF
}

impl Percolation {
    pub fn new(n: usize) -> Percolation {
        assert!(n > 0);
        let num_sites = n * n;
        Percolation { n : n,
                      num_sites: n * n,
                      sites: vec![false; num_sites],
                      uf_with_top_bottom: WeightedQuickUnion::new(num_sites + 2),
                      uf_with_top_only: WeightedQuickUnion::new(num_sites+1)}
    }

    fn index(&self, i: usize, j: usize) -> usize {
        (i-1) * self.n + j - 1
    }

    fn top_index(&self) -> usize {
        self.num_sites
    }

    fn bottom_index(&self) -> usize {
        self.num_sites + 1
    }

    pub fn open(&mut self, i: usize, j: usize) {
        assert!(i >= 1 && i <= self.n && j >=1 && j <= self.n);

        let index = self.index(i, j);
        self.sites[index] = true;

        //i, j - 1
        if j > 1 {
            let lindex = self.index(i, j - 1);
            if self.sites[lindex] {
                self.uf_with_top_bottom.union(index, lindex);
                self.uf_with_top_only.union(index, lindex);
            }
        }

        //i, j + 1
        if j < self.n {
            let rindex = self.index(i, j + 1);
            if self.sites[rindex] {
                self.uf_with_top_bottom.union(index, rindex);
                self.uf_with_top_only.union(index, rindex);
            }
        }

        //i-1,j
        if i > 1 {
            let tindex = self.index(i - 1, j);
            if self.sites[tindex] {
                self.uf_with_top_bottom.union(index, tindex);
                self.uf_with_top_only.union(index, tindex);
            }
        }

        //i+1,j
        if i < self.n {
            let bindex = self.index( i + 1, j);
            if self.sites[bindex] {
                self.uf_with_top_bottom.union(index, bindex);
                self.uf_with_top_only.union(index, bindex);
            }
        }

        //connect with virtual site(s)
        let top_index = self.top_index();

        if i == 1 {
            self.uf_with_top_bottom.union(index, top_index);
            self.uf_with_top_only.union(index, top_index);
        }

        let bottom_index = self.bottom_index();

        if i == self.n {
            self.uf_with_top_bottom.union(index, bottom_index);
        }
    }

    pub fn is_open(&self, i: usize, j: usize) -> bool {
        assert!(i >= 1 && i <= self.n && j >= 1 && j <= self.n);

        let index = self.index(i, j);
        self.sites[index]
    }

    pub fn is_full(&self, i: usize, j: usize) -> bool {
        assert!(i >= 1 && i <= self.n && j >=1 && j <= self.n);

        let index = self.index(i, j);

        self.uf_with_top_only.connected(index, self.top_index())
    }

    pub fn percolates(&self) -> bool {
        self.uf_with_top_bottom.connected(self.top_index(), self.bottom_index())
    }
}
