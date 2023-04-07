impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let length = nums.len();
        if length == 0 {
            return false;
        }

        let mut last_reachable_index = length - 1;
        for i in (0..length).rev() {
            if i + nums[i] as usize >= last_reachable_index {
                last_reachable_index = i;
            }
        }

        last_reachable_index == 0
    }
}
