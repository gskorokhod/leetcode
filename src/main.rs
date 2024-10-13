mod add_two_numbers;
use add_two_numbers::add_two_numbers;

fn main() {
    // Problem 1: https://leetcode.com/problems/add-two-numbers
    let a = add_two_numbers::helpers::parse("342");
    assert_eq!(add_two_numbers::helpers::to_vec(&a), vec![2, 4, 3]);
    let b = add_two_numbers::helpers::parse("465");
    assert_eq!(add_two_numbers::helpers::to_vec(&b), vec![5, 6, 4]);
    let ans = add_two_numbers(a, b);
    assert_eq!(add_two_numbers::helpers::to_vec(&ans), vec![7, 0, 8]);

    // Problem 2: to be continued...
}
