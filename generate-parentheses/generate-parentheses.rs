impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn generate(n: i32, left: i32, right: i32, s: &mut String, result: &mut Vec<String>) {
            if left == n && right == n {
                result.push(s.clone());
            } else {
                if left < n {
                    s.push('(');
                    println!("{}, l<n, {left}, {n}",s);
                    generate(n, left + 1, right, s, result);
                    s.pop();
                }
                if right < left {
                    s.push(')');
                    println!("{}, r<l, {right}, {n}",s);
                    generate(n, left, right + 1, s, result);
                    s.pop();
                }
            }
        }
        let mut result = vec![];
        let mut s = String::new();
        generate(n, 0, 0, &mut s, &mut result);
        result
    }
}
