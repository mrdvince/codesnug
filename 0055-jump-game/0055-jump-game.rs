impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i > max_reachable {
                return false;
            }
            max_reachable = max_reachable.max(i + num as usize);
            if max_reachable >= nums.len() - 1 {
                return true;
            }
        }
        false
    }
}
