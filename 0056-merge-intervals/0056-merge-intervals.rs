impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        
        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut result: Vec<Vec<i32>> = Vec::new();
        result.push(intervals[0].clone());
        
        for i in 1..intervals.len() {
            let last_interval = result.last_mut().unwrap();
            if last_interval[1] < intervals[i][0] {
                result.push(intervals[i].clone());
            } else {
                last_interval[1] = last_interval[1].max(intervals[i][1]);
            }
        }
        
        result
    }
}
