impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return Vec::new();
        }
        
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut result: Vec<Vec<i32>> = Vec::new();
        result.push(intervals[0].clone());
        
        for interval in &intervals[1..] {
            let last_interval = result.last_mut().unwrap();
            if last_interval[1] < interval[0] {
                result.push(interval.clone());
            } else {
                last_interval[1] = last_interval[1].max(interval[1]);
            }
        }
        
        result
    }
}
