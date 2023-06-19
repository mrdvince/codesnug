impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        nums.sort();
        for (idx, num) in nums.iter().enumerate() {
            if idx > 0 && num == &nums[idx - 1] {
                continue;
            }
            let (mut left, mut right) = (idx + 1, nums.len() - 1);

            while left < right {
                let threesum = num + nums[left] + nums[right];

                if threesum > 0 {
                    right -= 1;
                } else if threesum < 0 {
                    left += 1;
                } else {
                    result.push(vec![*num, nums[left], nums[right]]);
                    left += 1;
                    while nums[left] == nums[left - 1] && left < right {
                        left += 1;
                    }
                }
            }
        }
        result
    }
}
