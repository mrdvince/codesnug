impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_so_far = nums[0];
        let mut max_ending_here = nums[0];
        for i in 1..nums.len() {
            max_ending_here = nums[i].max(max_ending_here + nums[i]);
            max_so_far = max_so_far.max(max_ending_here);
        }
        max_so_far
    }
}
