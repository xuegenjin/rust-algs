use std::cmp::PartialOrd;

pub fn shell_sort<T: PartialOrd>(a: &mut [T]) {
    let n = a.len();

    let mut h = 1;
    while h < n / 3 {
        h = 3 * h + 1;
    }

    while h >= 1 {
        for i in h..n {
            let mut j = i;
            while j >= h && a[j] < a[j-h] {
                a.swap(j, j-h);
                j -= h;
            }
        }

        h = h/3;
    }
}
