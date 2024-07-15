impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        while l < r {
            let area = (r - l) as i32 * height[l].min(height[r]);
            ans = ans.max(area);
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }

        }
        ans
    }
}