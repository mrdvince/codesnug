impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};

        let (mut left_ptr, mut right_ptr, mut max_area) = (0, height.len() - 1, 0);

        while left_ptr < right_ptr {
            let h = min(height[left_ptr], height[right_ptr]);
            let w = (right_ptr - left_ptr) as i32;

            max_area = max(max_area, h * w);

            if height[left_ptr] < height[right_ptr] {
                left_ptr += 1;
            } else {
                right_ptr -= 1;
            }
        }

        max_area
    }
}
