use crate::Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        // Local variable declarations
        let mut parenthesis_stack: usize = 0;
        let mut result: Vec<String> = Vec::new();
        let mut string: String = String::new();

        // Begin algorithm
        s.chars().for_each(|ch| {
            match ch {
                '(' => {
                    parenthesis_stack += 1;
                    string.push('(');
                }
                ')' => {
                    parenthesis_stack -= 1;
                    string.push(')');
                }
                _ => {
                    panic!("unexpected character:'{}'. Please verify the input", ch);
                }
            };
            match parenthesis_stack {
                0 => {
                    let s = string[1..string.len() - 1].parse().unwrap();
                    result.push(s);
                    string.clear();
                }
                _ => {}
            }
        });
        result
            .iter()
            .fold(String::new(), |acc, x| format!("{}{}", acc, x))
    }
}

#[test]
fn problem_1021_test() {
    assert_eq!(
        Solution::remove_outer_parentheses(String::from("(()())(())")),
        "()()()"
    );
    assert_eq!(Solution::remove_outer_parentheses(String::from("()()")), "");
    assert_eq!(
        Solution::remove_outer_parentheses(String::from("(()())(())(()(()))")),
        "()()()()(())"
    );
}
