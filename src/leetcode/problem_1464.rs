use crate::Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.iter().rev().take(2).fold(1, |acc, x| acc * (x - 1))
    }
}

#[test]
pub fn test_contest_5424() {
    let input = vec![3, 4, 5, 2];
    assert_eq!(12, Solution::max_product(input));
}
