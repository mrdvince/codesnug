impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.is_empty() {
            return;
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut i = 0;

        while i <= right {
            match nums[i] {
                0 => {
                    nums.swap(i, left);
                    left += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, right);
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
                _ => i += 1,
            }
        }
    }
}


// impl Solution {
//     pub fn sort_colors(nums: &mut Vec<i32>) {
//         let mut counts = [0; 3];

//         for &num in nums.iter() {
//             counts[num as usize] += 1;
//         }
//         let mut j = 0;
//         for i in 0..3 {
//             for _ in 0..counts[i] {
//                 nums[j] = i as i32;
//                 j += 1
//             }
//         }
//     }
// }