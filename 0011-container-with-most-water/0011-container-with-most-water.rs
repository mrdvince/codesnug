impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = height.len() - 1;
        let mut max_area = 0;

        while left_ptr < right_ptr {
            let base = right_ptr - left_ptr;
            let curr_area = height[left_ptr].min(height[right_ptr]) * base as i32;
            max_area = max_area.max(curr_area);

            if height[left_ptr] < height[right_ptr] {
                left_ptr += 1;
            } else {
                right_ptr -= 1;
            }
        }
        max_area
    }
}
