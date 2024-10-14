fn row(board: &Vec<Vec<char>>, idx: usize) -> Vec<&char> {
    return board[idx]
        .iter()
        .filter(|&x| *x != '.')
        .collect::<Vec<&char>>();
}

fn col(board: &Vec<Vec<char>>, idx: usize) -> Vec<&char> {
    return board
        .iter()
        .map(|x| &x[idx])
        .filter(|&x| *x != '.')
        .collect::<Vec<&char>>();
}

fn square(board: &Vec<Vec<char>>, n: usize) -> Vec<&char> {
    return board
        .iter()
        .skip((n / 3) * 3)
        .take(3)
        .flat_map(|x| x.iter().skip((n % 3) * 3).take(3))
        .filter(|&x| *x != '.')
        .collect::<Vec<&char>>();
}

fn valid(val: &Vec<&char>) -> bool {
    let mut set = std::collections::HashSet::new();
    for &x in val {
        if set.contains(x) {
            return false;
        }
        set.insert(x);
    }
    return true;
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    for i in 0..9 {
        if !valid(&row(&board, i)) || !valid(&col(&board, i)) || !valid(&square(&board, i)) {
            return false;
        }
    }
    return true;
}
