use crate::Solution;

impl Solution {
    pub fn reverse(input_value: i32) -> i32 {
        let mut reversed_integer: i32 = 0;
        let mut pow_10 = input_value;
        while pow_10 != 0 {
            match reversed_integer.checked_mul(10) {
                Some(multiplied) => {
                    let remainder = pow_10 % 10;
                    reversed_integer = multiplied + remainder;
                    pow_10 /= 10;
                }
                None => {
                    return 0;
                }
            }
        }
        reversed_integer
    }
}

#[test]
fn problem_7_test() {
    assert_eq!(Solution::reverse(1234), 4321);
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(12), 21);
    assert_eq!(Solution::reverse(345), 543);
    assert_eq!(Solution::reverse(765), 567);
    assert_eq!(Solution::reverse(321), 123);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1), 1);
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(-1234), -4321);
    assert_eq!(Solution::reverse(-2147483648), 0);
}
