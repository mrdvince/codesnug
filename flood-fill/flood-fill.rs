impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (rows, cols) = (image.len(), image[0].len());
        let (sr, sc) = (sr as usize, sc as usize);
        let mut image = image;
        let old_color = image[sr][sc];

        if old_color == color {
            return image;
        }

        let mut stack = Vec::new();
        stack.push((sr, sc));

        while let Some((r, c)) = stack.pop() {
            if r >= rows || c >= cols || image[r][c] != old_color {
                continue;
            }

            // set new color
            image[r][c] = color;

            // Add neighbors to the stack
            stack.push((r - 1, c)); // up
            stack.push((r + 1, c)); // down
            stack.push((r, c - 1)); // left
            stack.push((r, c + 1)); // right
        }

        image
    }
}
