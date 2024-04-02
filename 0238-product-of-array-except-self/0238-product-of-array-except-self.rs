impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut ans = vec![1; len];
        // Product of elements to the left of i
        let mut left = 1;
        for idx in 0..len {
            ans[idx] = left;
            left *= nums[idx];
        }
        // From the right plus multiply with elemens in ans
        let mut right = 1;
        for idx in (0..len).rev() {
            ans[idx] *= right;
            right *= nums[idx];
        }
        ans
    }
}
