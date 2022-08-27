impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut t = 0;
        let mut col = vec![0; 9];
        let mut row = vec![0; 9];
        let mut cell = vec![0; 9];
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                t = 1 << (board[i][j] as i32 - '0' as i32);
                let k = i / 3 * 3 + j / 3;
                if t & col[i] > 0 || t & row[j] > 0 || t & cell[k] > 0 {
                    return false;
                }
                col[i] |= t;
                row[j] |= t;
                cell[k] |= t;
            }
        }
        return true;
    }
}
