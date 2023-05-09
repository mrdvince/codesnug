impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut m_element = 0;
        let mut count = 0;
        for num in nums {
            if count == 0 {
                m_element = num;
                count = 1;
            } else if m_element == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        m_element
    }
}