#[allow(dead_code)]
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
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
    let n = board.len();
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
    }

    // check right above
    {
        let mut i = row as i32 - 1;
        let mut j = col + 1;
        while i >= 0 && j < n {
            if board[i as usize][j] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
    }
    // check left above
    {
        let mut i = row as i32 - 1;
        let mut j = col as i32 - 1;
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
    }
    return true;
}

#[test]
fn nqueens_test() {
    let a = solve_n_queens(4);
    println!("{:?}", a);
}
