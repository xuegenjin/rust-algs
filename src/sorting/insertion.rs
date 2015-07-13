use std::cmp::PartialOrd;

pub fn insertion_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && a[j] < a[j-1] {
            a.swap(j, j-1);
            j -= 1;
        }
    }
}
