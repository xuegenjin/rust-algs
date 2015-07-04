pub trait UF {
    fn find(&self, p:usize) -> usize;

    fn union (&mut self, p:usize, q: usize);

    fn connected(&self, p: usize, q: usize) -> bool {
        self.find(p) == self.find(q)
    }

    fn count(&self) -> usize;
}

#[macro_export]
macro_rules! impl_path_methods {
    () => (
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
    )
}
