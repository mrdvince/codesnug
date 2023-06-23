use std::collections::BinaryHeap;
// O(n log k) time O(k) space
impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let k = k as usize;
        for point in points {
            let dist = point[0].pow(2) + point[1].pow(2);
            if heap.len() < k {
                heap.push((dist, point));
            } else if let Some(&(top_dist, _)) = heap.peek() {
                if top_dist > dist {
                    heap.pop();
                    heap.push((dist, point));
                }
            }
        }
        heap.into_iter().map(|(_, point)| point).collect()
    }
}
