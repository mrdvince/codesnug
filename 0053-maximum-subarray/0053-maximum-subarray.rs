impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let (first, rest) = nums.split_first().unwrap();
        let mut max_so_far = *first;
        let mut max_ending_here = *first;
        
        for &num in rest {
            max_ending_here = num.max(max_ending_here + num);
            max_so_far = max_so_far.max(max_ending_here);
        }
        
        max_so_far
    }
}

// Divide and conquer
// impl Solution {
//     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//         Self::helper(&nums, 0, nums.len() - 1)
//     }
    
//     fn helper(nums: &[i32], left: usize, right: usize) -> i32 {
//         if left == right {
//             return nums[left];
//         }
        
//         let mid = (left + right) / 2;
//         let left_sum = Self::helper(nums, left, mid);
//         let right_sum = Self::helper(nums, mid + 1, right);
//         let cross_sum = Self::cross_sum(nums, left, right, mid);
        
//         left_sum.max(right_sum).max(cross_sum)
//     }
    
//     fn cross_sum(nums: &[i32], left: usize, right: usize, mid: usize) -> i32 {
//         if left == right {
//             return nums[left];
//         }
        
//         let mut left_subsum = i32::MIN;
//         let mut curr_sum = 0;
//         for i in (left..=mid).rev() {
//             curr_sum += nums[i];
//             left_subsum = left_subsum.max(curr_sum);
//         }
        
//         let mut right_subsum = i32::MIN;
//         curr_sum = 0;
//         for i in mid + 1..=right {
//             curr_sum += nums[i];
//             right_subsum = right_subsum.max(curr_sum);
//         }
        
//         left_subsum + right_subsum
//     }
// }
