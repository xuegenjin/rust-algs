pub trait Bag<T> {
    fn add(item: T);
    fn is_empty() -> bool;
    fn size() -> usize;
}
