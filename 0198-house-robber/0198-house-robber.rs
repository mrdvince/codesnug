impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut loot1, mut loot2) = (0, 0);
        
        for num in nums {
            let temp = loot1;
            loot1 = std::cmp::max(loot2 + num, loot1);
            loot2 = temp;
        }
        loot1
    }
}