impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counts = [0; 3];

        for &num in nums.iter() {
            counts[num as usize] += 1;
        }
        let mut j = 0;
        for i in 0..3 {
            for _ in 0..counts[i] {
                nums[j] = i as i32;
                j += 1
            }
        }
    }
}