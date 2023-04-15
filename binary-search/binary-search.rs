impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: i32 = 0;
        let mut hi = (nums.len() - 1) as i32;

        while lo <= hi {
            let mid = (lo + hi) / 2;
            if target == nums[mid as usize] {
                return mid;
            }
            if nums[mid as usize] < target {
                lo = mid + 1;
            } else {
                hi = mid - 1
            }
        }
        -1
    }
}