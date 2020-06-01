use crate::Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        match n {
            0 | 1 | 2 => 1,
            3 => 2,
            _ => {
                let mut dp: Vec<i32> = vec![1; (n as usize) + 1];
                dp[2] = 2;
                dp[3] = 3;
                for x in 3..=n {
                    for y in 1..=x / 2 {
                        let x = x as usize;
                        let y = y as usize;
                        let left_index = x - y;
                        let right_index = y as usize;
                        let left_value = dp[left_index];
                        let right_value = dp[right_index];
                        let new_value = left_value * right_value;
                        dp[x] = std::cmp::max(dp[x], new_value);
                    }
                }
                dp[n as usize] as i32
            }
        }
    }

    //  TODO    ::  Implement recursion version of this.
    pub fn integer_break_recursive(_: i32) -> i32 {
        0
    }
}

#[test]
pub fn problem_343_test() {
    assert_eq!(Solution::integer_break(2), 1);
    assert_eq!(Solution::integer_break(4), 4);
    assert_eq!(Solution::integer_break(6), 9);
    assert_eq!(Solution::integer_break(10), 36);
}
