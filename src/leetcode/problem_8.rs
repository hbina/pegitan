use crate::Solution;

// TODO :: Implement this...
impl Solution {
    pub fn my_atoi<T>(_: T) -> i32
    where
        T: Into<String>,
    {
        0
    }
}

#[test]
fn problem_8_failing() {
    assert_eq!(Solution::my_atoi("4193 with words"), 4193);
    assert_eq!(Solution::my_atoi("words and 987"), 987);
    assert_eq!(Solution::my_atoi("   -42"), -42);
    assert_eq!(Solution::my_atoi("42"), 42);
    assert_eq!(Solution::my_atoi("-91283472332"), -2147483648);
}
