extern crate algs4;

use algs4::sorting::*;

fn test_ints(sort: fn(&mut [usize]) -> ()) {
    let mut a = vec![7, 10, 5, 3, 8, 4, 2, 9, 6];

    sort(&mut a);

    assert_eq!(a, [2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

fn test_strings(sort: fn(&mut[&'static str])->()) {
    let mut a = vec!["S", "O", "R", "T", "E", "X", "A", "M", "P", "L", "E" ];

    sort(&mut a);

    assert_eq!(a, ["A", "E", "E", "L", "M", "O", "P", "R", "S", "T", "X"]);
}

fn test_strings2(sort: fn(&mut[&'static str])->()) {
    let mut a = vec!["S", "H", "E", "L", "L", "S", "O", "R", "T", "E", "X", "A", "M", "P", "L", "E" ];

    sort(&mut a);

    assert_eq!(a, ["A", "E", "E", "E", "H", "L", "L", "L", "M", "O", "P", "R", "S", "S", "T", "X"]);
}


#[test]
fn selection_tests() {
    test_ints(selection_sort);
    test_strings(selection_sort);
}

#[test]
fn insertion_tests() {
    test_ints(insertion_sort);
    test_strings(insertion_sort);
}


#[test]
fn shell_tests() {
    test_ints(shell_sort);
    test_strings(shell_sort);
    test_strings2(shell_sort);
}
