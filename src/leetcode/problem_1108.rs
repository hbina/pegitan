use crate::Solution;

impl Solution {
    pub fn defang_i_paddr<T>(address: T) -> String
    where
        T: Into<String>,
    {
        address
            .into()
            .chars()
            .into_iter()
            .fold(String::new(), |acc, c| match c {
                '.' => format!("{}{}", acc, "[.]"),
                _ => format!("{}{}", acc, c),
            })
    }
}

#[test]
fn problem_1108_test() {
    assert_eq!(Solution::defang_i_paddr("1.1.1.1"), "1[.]1[.]1[.]1");
}
