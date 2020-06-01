use crate::Solution;

impl Solution {
    pub fn is_subsequence<T>(s: T, t: T) -> bool
    where
        T: Into<String>,
    {
        let mut stack = s.into().chars().collect::<std::collections::VecDeque<_>>();
        let expected_length = stack.len();
        let t = t.into();
        t.chars().fold(0, |mut acc, x| {
            let top = stack.front();
            match top {
                Some(&some) => {
                    if some == x {
                        stack.pop_front();
                        acc = acc + 1;
                    }
                }
                None => {}
            }
            acc
        }) == expected_length
    }
}

#[test]
fn problem_392_test() {
    assert_eq!(Solution::is_subsequence("abc", "ahbgdc"), true);
    assert_eq!(Solution::is_subsequence("axc", "ahbgdc"), false);
}
