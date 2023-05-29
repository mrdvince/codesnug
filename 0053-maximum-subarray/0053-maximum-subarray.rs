impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.iter().max().unwrap() < &0 {
            return *nums.iter().max().unwrap();
        }

        let mut max_sum = nums[0];
        let mut curr_sum = 0;

        for num in nums {
            if curr_sum < 0 {
                curr_sum = 0;
            }
            curr_sum += num;
            max_sum = std::cmp::max(max_sum, curr_sum);
            println!("{}, {}", max_sum, curr_sum);
        }
        max_sum
    }
}
