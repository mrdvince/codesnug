use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for pair in prerequisites {
            graph.entry(pair[0]).or_insert(Vec::new()).push(pair[1]);
        }
        
        let mut visited: HashMap<i32, i32> = HashMap::new();
        
        for i in 0..num_courses {
            if Solution::dfs(i, &mut visited, &graph) {
                return false;
            }
        }
        
        true
    }
    
    fn dfs(node: i32, visited: &mut HashMap<i32, i32>, graph: &HashMap<i32, Vec<i32>>) -> bool {
        if let Some(&status) = visited.get(&node) {
            return status == 1;
        }
        
        visited.insert(node, 1);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if Solution::dfs(neighbor, visited, graph) {
                    return true;
                }
            }
        }
        
        visited.insert(node, -1);
        false
    }
}
