pub trait Stack<T> {
    fn push(item: T);
    fn pop() -> T;
    fn is_empty() -> bool;
    fn size() -> usize;
}
