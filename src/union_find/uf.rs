pub trait UF {
    fn find(&self, p:usize) -> usize;

    fn union (&mut self, p:usize, q: usize);

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn count(&self) -> usize;
}
