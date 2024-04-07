impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use::std::collections::HashSet;
        let mut rows = vec![HashSet::new();9];
        let mut cols = vec![HashSet::new();9];
        let mut squares = vec![HashSet::new();9];

        for r in 0..9 {
            for c in 0..9{
                let curr = board[r][c];
                if curr == '.'{
                    continue;
                }
                // the 3x3 grid
                let square_index = (r/3) * 3 + c /3;
                // println!("{:?}, \n {:?}, \n {:?}\n\n", rows,cols,squares);
                if rows[r].contains(&curr) || cols[c].contains(&curr) || squares[square_index].contains(&curr){
                    return false;
                }
                rows[r].insert(curr);
                cols[c].insert(curr);
                squares[square_index].insert(curr);
            }
        }
        true
    }
}