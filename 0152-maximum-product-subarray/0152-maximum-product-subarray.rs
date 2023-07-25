impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut min_so_far = nums[0];
        let mut result = nums[0];
        
        for i in 1..nums.len() {
            let curr = nums[i];
            let max_product = max_so_far * curr;
            let min_product = min_so_far * curr;
            let temp_max = max_so_far;
            max_so_far = std::cmp::max(std::cmp::max(max_product, min_product), curr);
            min_so_far = std::cmp::min(std::cmp::min(max_product, min_product), curr);
            if max_so_far > result {
                result = max_so_far;
            }
        }
        result
    }
}
