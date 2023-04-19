impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let rows = image.len();
        let cols = image[0].len();
        let mut image = image;
        let sr = sr as usize;
        let sc = sc as usize;
        let old_color = image[sr][sc];

        if old_color == color {
            return image;
        }

        Self::dfs(&mut image, sr, sc, old_color, color, rows, cols);
        image
    }
    fn dfs(
        image: &mut Vec<Vec<i32>>,
        sr: usize,
        sc: usize,
        old_color: i32,
        color: i32,
        rows: usize,
        cols: usize,
    ) {
        if sr < 0 || sr >= rows || sc < 0 || sc >= cols || image[sr][sc] != old_color {
            return;
        }

        // set new color
        image[sr][sc] = color;
        // left
        Self::dfs(image, sr - 1, sc, old_color, color, rows, cols);
        // right
        Self::dfs(image, sr + 1, sc, old_color, color, rows, cols);
        // up
        Self::dfs(image, sr, sc - 1, old_color, color, rows, cols);
        // down
        Self::dfs(image, sr, sc + 1, old_color, color, rows, cols);
    }
}
