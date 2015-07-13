pub trait Queue<T> {
    fn enqueue(item: T);
    fn dequeue() -> T;
    fn is_empty() -> bool;
    fn size() -> usize;
}
