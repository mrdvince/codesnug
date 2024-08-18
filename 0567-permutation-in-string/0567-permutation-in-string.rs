impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let mut s1_freq = vec![0; 26];
        let mut window_freq = vec![0; 26];

        // Build frequency vector for s1
        for ch in s1.bytes() {
            s1_freq[(ch - b'a') as usize] += 1;
        }

        // Sliding window on s2
        let s2_bytes: Vec<u8> = s2.bytes().collect();
        for i in 0..s2.len() {
            // Add right char to window
            window_freq[(s2_bytes[i] - b'a') as usize] += 1;

            // Remove left char from window if window size > s1.len()
            if i >= s1.len() {
                window_freq[(s2_bytes[i - s1.len()] - b'a') as usize] -= 1;
            }

            // Check if window frequency matches s1 frequency
            if i >= s1.len() - 1 && s1_freq == window_freq {
                return true;
            }
        }

        false
    }
}