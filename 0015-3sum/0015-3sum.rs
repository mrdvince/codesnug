impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let len = nums.len();
        let mut ans = vec![];

        for i in 0..len {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let (mut li, mut ri) = (i + 1, nums.len() - 1);
            while li < ri {
                let three_sum = nums[i] + nums[li] + nums[ri];
                if three_sum == 0 {
                    ans.push(vec![nums[i], nums[li], nums[ri]]);
                    // keeep looking
                    li += 1;
                    while li < ri && nums[li] == nums[li - 1] {
                        li += 1;
                    }
                    ri -= 1;
                    while li < ri && nums[ri] == nums[ri + 1] {
                        ri -= 1;
                    }
                } else if three_sum > 0 {
                    ri -= 1;
                } else if three_sum < 0 {
                    li += 1;
                }
            }
        }
        ans
    }
}
