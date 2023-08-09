// // TC: O(c⋅t⋅2 ^t)
// impl Solution {
//     pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//         let target = target as usize;
//         let mut dp: Vec<Vec<Vec<i32>>> = vec![Vec::new(); target + 1];
//         dp[0].push(vec![]);
        
//         for &candidate in &candidates {
//             let candidate = candidate as usize;
//             for i in candidate..=target {
//                 let mut new_combinations = Vec::new();
//                 for previous_combination in &dp[i - candidate] {
//                     let mut new_combination = previous_combination.clone();
//                     new_combination.push(candidate as i32);
//                     new_combinations.push(new_combination);
//                 }
//                 dp[i].append(&mut new_combinations);
//             }
//         }
        
//         dp[target].clone()
//     }
// }

// DFS
// TC: O(N^T)
// impl Solution {
//     pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//         let mut result = Vec::new();
//         Solution::dfs(&candidates, target, 0, &mut Vec::new(), &mut result);
//         result
//     }
//     fn dfs(
//         candidates: &Vec<i32>,
//         target: i32,
//         start: usize,
//         current: &mut Vec<i32>,
//         result: &mut Vec<Vec<i32>>,
//     ) {
//         if target == 0 {
//             result.push(current.clone());
//             return;
//         }
//         for i in start..candidates.len() {
//             if candidates[i] <= target {
//                 current.push(candidates[i]);
//                 Solution::dfs(candidates, target - candidates[i], i, current, result);
//                 current.pop();
//             }
//         }
//     }
// }

// Bactrack
// TC: O(N^T)
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Solution::backtrack(&candidates, target, 0, &mut combination, &mut result);
        result
    }
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(combination.clone());
            return;
        }
        if target < 0 {
            return;
        }
        for i in start..candidates.len() {
            combination.push(candidates[i]);
            Solution::backtrack(candidates, target - candidates[i], i, combination, result);
            combination.pop();
        }
    }
}
