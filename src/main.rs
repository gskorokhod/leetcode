mod add_two_numbers;
use add_two_numbers::add_two_numbers;
mod is_valid_sudoku;
use is_valid_sudoku::is_valid_sudoku;

fn main() {
    // Problem 1: https://leetcode.com/problems/add-two-numbers
    let a = add_two_numbers::helpers::parse("342");
    assert_eq!(add_two_numbers::helpers::to_vec(&a), vec![2, 4, 3]);
    let b = add_two_numbers::helpers::parse("465");
    assert_eq!(add_two_numbers::helpers::to_vec(&b), vec![5, 6, 4]);
    let ans = add_two_numbers(a, b);
    assert_eq!(add_two_numbers::helpers::to_vec(&ans), vec![7, 0, 8]);

    // Problem 2: https://leetcode.com/problems/is-valid-sudoku
    let board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert!(is_valid_sudoku(board));
}
