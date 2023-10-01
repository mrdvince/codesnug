impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = (k as usize) % n;

        // Reverse the whole array
        nums.reverse();

        // Reverse the first k elements
        nums[0..k].reverse();

        // Reverse the remaining n - k elements
        nums[k..].reverse();
    }
}
