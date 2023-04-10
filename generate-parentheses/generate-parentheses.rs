impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        stack.push((0, 0, String::new()));

        while let Some((left, right, s)) = stack.pop() {
            if left == n && right == n {
                result.push(s);
            } else {
                if left < n {
                    let mut new_s = s.clone();
                    new_s.push('(');
                    stack.push((left + 1, right, new_s));
                }
                if right < left {
                    let mut new_s = s.clone();
                    new_s.push(')');
                    stack.push((left, right + 1, new_s));
                }
            }
        }

        result
    }
}
