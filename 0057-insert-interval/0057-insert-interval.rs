impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut start, mut end) = (new_interval[0], new_interval[1]);
        let mut i = 0;
        let mut result: Vec<Vec<i32>> = Vec::new();

        // add all the intervals ending before start of new_interval to result
        while i < intervals.len() && intervals[i][1] < start {
            result.push(intervals[i].clone());
            i += 1;
        }

        // merge all overlapping intervals
        while i < intervals.len() && intervals[i][0] <= end {
            start = std::cmp::min(start, intervals[i][0]);
            end = std::cmp::max(end, intervals[i][1]);
            i += 1;
        }
        result.push(vec![start, end]);

        // add remaining intervals
        while i < intervals.len() {
            result.push(intervals[i].clone());
            i += 1;
        }

        result
    }
}
