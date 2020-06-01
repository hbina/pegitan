use crate::Solution;

impl Solution {
    pub fn is_palindrome(input_value: i32) -> bool {
        match input_value.cmp(&0) {
            std::cmp::Ordering::Less => false,
            _ => {
                let mut reversed_integer = 0;
                let mut pow_10 = input_value;
                while pow_10 != 0 {
                    let remainder = pow_10 % 10;
                    reversed_integer = reversed_integer * 10 + remainder;
                    pow_10 /= 10;
                }
                reversed_integer == input_value
            }
        }
    }
}

#[test]
fn problem_9_test() {
    assert_eq!(Solution::is_palindrome(32123), true);
    assert_eq!(Solution::is_palindrome(4334), true);
    assert_eq!(Solution::is_palindrome(1221), true);
    assert_eq!(Solution::is_palindrome(0), true);
    assert_eq!(Solution::is_palindrome(1321), false);
    assert_eq!(Solution::is_palindrome(9), true);
    assert_eq!(Solution::is_palindrome(91), false);
    assert_eq!(Solution::is_palindrome(-121), false);
}
