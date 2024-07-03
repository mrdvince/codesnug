impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut li = 0;
        let mut ri = numbers.len() - 1;

        while li < ri {
            let curr_sum = numbers[li] + numbers[ri];
            if curr_sum == target {
                return vec![(li + 1) as i32, (ri + 1) as i32];
            }
            if curr_sum < target {
                li += 1;
            }
            if curr_sum > target {
                ri -= 1;
            }
        }
        unreachable!("never gets here")
    }
}
