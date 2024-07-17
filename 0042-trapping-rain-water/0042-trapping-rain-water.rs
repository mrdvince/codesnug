impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 0 {
            return 0;
        }
        let mut res = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);
        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                res += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                res += right_max - height[right];
            }
        }
        res
    }
}
