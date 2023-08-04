impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut new_interval = new_interval;

        for interval in intervals.into_iter() {
            if new_interval[1] < interval[0] {
                result.push(new_interval.clone());
                new_interval = interval;
            } else if interval[1] < new_interval[0] {
                result.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        }

        result.push(new_interval);
        result
    }
}


// impl Solution {
//     pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut left = 0;
//         let mut right = intervals.len();
        
//         while left < right {
//             let mid = left + (right - left) / 2;
//             if intervals[mid][0] < new_interval[0] {
//                 left = mid + 1;
//             } else {
//                 right = mid;
//             }
//         }
        
//         let mut intervals = intervals;
//         intervals.insert(left, new_interval);
        
//         let mut result = vec![intervals[0].clone()];
//         for interval in intervals.into_iter().skip(1) {
//             if interval[0] > result.last().unwrap()[1] {
//                 result.push(interval);
//             } else {
//                 result.last_mut().unwrap()[1] = result.last().unwrap()[1].max(interval[1]);
//             }
//         }
//         result
//     }
// }
