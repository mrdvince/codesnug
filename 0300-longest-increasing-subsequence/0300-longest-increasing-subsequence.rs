impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lengths = vec![1; n];
        
        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lengths[i] = lengths[i].max(lengths[j] + 1);
                }
            }
        }
        
        *lengths.iter().max().unwrap()
    }
}
