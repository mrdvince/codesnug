impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1; n];
        let mut left_product = 1;
        
        // calculate left products
        for i in 0..n {
            output[i] *= left_product;
            left_product *= nums[i];
        }
        
        let mut right_product = 1;
        // calculate right products
        for i in (0..n).rev() {
            output[i] *= right_product;
            right_product *= nums[i];
        }
        
        output
    }
}
