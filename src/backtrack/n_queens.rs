#[allow(dead_code)]
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>>  {
    let mut res = vec![];
    let mut board = vec![vec!['.'; n as usize]; n as usize];
    backtrack(&mut board, 0, &mut res);
    res
}

fn backtrack(board: &mut Vec<Vec<char>>, row: usize, res: &mut Vec<Vec<String>>) {
    if row as usize == board.len() {
        let mut result = Vec::new();
        for item in board {
            let s = item.iter().collect();
            result.push(s);
        }
        res.push(result);
        return;
    }

    let n = board[row].len();
    for col in 0..n {
        if !is_valid(board, row, col) {
            continue;
        }

        board[row][col] = 'Q';
        backtrack(board, row + 1, res);
        board[row][col] = '.';

    }
}

fn is_valid(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    true
}