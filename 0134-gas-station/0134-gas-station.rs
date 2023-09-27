impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0;
        let mut curr_gas = 0;
        let mut start_idx = 0;

        for i in 0..gas.len() {
            total_gas += gas[i] - cost[i];
            curr_gas += gas[i] - cost[i];
            if curr_gas < 0 {
                start_idx = i + 1;
                curr_gas = 0;
            }
        }
        if total_gas < 0 {
            return -1;
        }
        start_idx as i32
    }
}