impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut li = 0;
        let mut ri = numbers.len() - 1;
        let mut ans: Vec<i32> = vec![];

        while li < ri {
            let curr_sum = numbers[li] + numbers[ri];
            if curr_sum == target {
                ans.append(&mut vec![
                    (li + 1).try_into().unwrap(),
                    (ri + 1).try_into().unwrap(),
                ]);
                break;
            }

            if curr_sum < target {
                li += 1;
            }
            if curr_sum > target {
                ri -= 1;
            }
        }
        ans
    }
}
