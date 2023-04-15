impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match nums.get(mid as usize) {
                Some(&mid_val) if mid_val == target => return mid as i32,
                Some(&mid_val) if mid_val < target => lo = mid + 1,
                _ => hi = mid - 1,
            }
        }
        -1
    }
}
