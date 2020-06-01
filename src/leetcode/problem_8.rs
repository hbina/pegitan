use crate::Solution;

// TODO: Need to solve for the cases where even i64 wouldn't fit...
impl Solution {
    pub fn my_atoi<T>(line: T) -> i32
    where
        T: Into<String>,
    {
        let result = line
            .into()
            .chars()
            .skip_while(|x| *x == ' ')
            .enumerate()
            .take_while(|(iter, x): &(usize, char)| match iter {
                0 => x.is_ascii_digit() || *x == '+' || *x == '-',
                _ => x.is_ascii_digit(),
            })
            .map(|x| x.1)
            .collect::<String>()
            .parse::<i64>()
            .unwrap_or(0);
        if result > std::i32::MAX as i64 {
            std::i32::MAX
        } else if result < std::i32::MIN as i64 {
            std::i32::MIN
        } else {
            result as i32
        }
    }
}

#[test]
fn problem_8_test_1() {
    assert_eq!(Solution::my_atoi("4193 with words"), 4193);
}

#[test]
fn problem_8_test_2() {
    assert_eq!(Solution::my_atoi("words and 987"), 0);
}

#[test]
fn problem_8_test_3() {
    assert_eq!(Solution::my_atoi("   -42"), -42);
}

#[test]
fn problem_8_test_4() {
    assert_eq!(Solution::my_atoi("42"), 42);
}

#[test]
fn problem_8_test_5() {
    assert_eq!(Solution::my_atoi("-91283472332"), -2147483648);
}

#[test]
fn problem_8_test_6() {
    assert_eq!(Solution::my_atoi("20000000000000000000"), 2147483647);
}
