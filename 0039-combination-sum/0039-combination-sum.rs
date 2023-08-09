// DFS
// TC: O(N^T)
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Solution::dfs(&candidates, target, 0, &mut Vec::new(), &mut result);
        result
    }
    fn dfs(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            result.push(current.clone());
            return;
        }
        for i in start..candidates.len() {
            if candidates[i] <= target {
                current.push(candidates[i]);
                Solution::dfs(candidates, target - candidates[i], i, current, result);
                current.pop();
            }
        }
    }
}

// Bactrack
// TC: O(N^T)
// impl Solution {
//     pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
//         let mut result = Vec::new();
//         let mut combination = Vec::new();
//         Solution::backtrack(&candidates, target, 0, &mut combination, &mut result);
//         result
//     }
//     fn backtrack(
//         candidates: &Vec<i32>,
//         target: i32,
//         start: usize,
//         combination: &mut Vec<i32>,
//         result: &mut Vec<Vec<i32>>,
//     ) {
//         if target == 0 {
//             result.push(combination.clone());
//             return;
//         }
//         if target < 0 {
//             return;
//         }
//         for i in start..candidates.len() {
//             combination.push(candidates[i]);
//             Solution::backtrack(candidates, target - candidates[i], i, combination, result);
//             combination.pop();
//         }
//     }
// }
