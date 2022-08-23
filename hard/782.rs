impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        fn count(vec: &Vec<i32>) -> i32 {
            let len = vec.len();
            let n = vec.iter().filter(|&&x| x == 1).count() as i32;
            let m = vec.iter().filter(|&&x| x == 0).count() as i32;
            if (n - m).abs() >= 2 {
                return -1;
            } else {
                let n1 = vec
                    .iter()
                    .enumerate()
                    .filter(|&(e, &n)| n == 1 - (e as i32 % 2))
                    .count();
                let n0 = vec
                    .iter()
                    .enumerate()
                    .filter(|&(e, &n)| n == e as i32 % 2)
                    .count();
                let num = match m - n {
                    -1 => len - n1,
                    0 => len - n1.max(n0),
                    1 => len - n0,
                    _ => 0,
                };
                return num as i32 / 2;
            }
        }
        let mut col = vec![];
        let mut row = &board[0];
        for line in board.iter() {
            if line != row && line != &row.iter().map(|&x| 1 - x).collect::<Vec<_>>() {
                return -1;
            } else {
                col.push((line == row) as i32);
            }
        }
        let ret_col = count(&col);
        let ret_row = count(&row);
        if ret_col < 0 || ret_row < 0 {
            -1
        } else {
            ret_row + ret_col
        }
    }
}
