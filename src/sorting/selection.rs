use std::cmp::PartialOrd;

pub fn selection_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();
    for i in 0..n {
        let mut min = i;

        for j in i+1 .. n {
            if a[j] < a[min] {
                min = j;
            }
        }

        a.swap(i, min);
    }
}
