// impl Solution {
//     pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//         let n = nums.len();
//         let k = (k as usize) % n;

//         // Reverse the whole array
//         nums.reverse();

//         // Reverse the first k elements
//         nums[0..k].reverse();

//         // Reverse the remaining n - k elements
//         nums[k..].reverse();
//     }
// }

// impl Solution {
//     pub fn rotate(nums: &mut Vec<i32>, k: i32) {
//         let mut start = 0;
//         let n = nums.len();
//         let k = k as usize % n;

//         let mut count = 0;
//         while count < n {
//             let mut current = start;
//             let mut prev = nums[start];
//             loop {
//                 let next = (current + k) % n;
//                 std::mem::swap(&mut prev, &mut nums[next]);
//                 current = next;
//                 count += 1;

//                 if start == current {
//                     break;
//                 }
//             }
//             start += 1;
//         }
//     }
// }

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        let mut result = vec![0; n];

        for i in 0..n {
            result[(i + k) % n] = nums[i];
        }

        *nums = result;
    }
}
