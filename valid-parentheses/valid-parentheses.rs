// Time complexity
// O(n), where n is the length of the input string s
// Space complexity
// O(n), e.g in the worst case (when s consists entirely of opening brackets)

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let brackets: std::collections::HashMap<char, char> = [(')', '('), ('}', '{'), (']', '[')]
            .iter()
            .cloned()
            .collect();
        if s.len() % 2 != 0 {
            return false;
        }
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
