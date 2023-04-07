impl Solution {
    pub fn is_valid(s: String) -> bool {
        let brackets: std::collections::HashMap<char, char> = [(')', '('), ('}', '{'), (']', '[')]
            .iter()
            .cloned()
            .collect();

        let mut stack = Vec::new();
        for bracket in s.chars() {
            match bracket {
                '(' | '{' | '[' => {
                    stack.push(bracket);
                }
                ')' | '}' | ']' => {
                    if stack.is_empty() || stack.last() != Some(&brackets[&bracket]) {
                        return false;
                    }
                    stack.pop();
                }
                _ => {}
            }
        }
        stack.is_empty()
    }
}
