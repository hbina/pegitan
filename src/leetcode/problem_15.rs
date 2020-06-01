use crate::Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();
        for x in 0..nums.len() {
            if x > 0 && nums[x] == nums[x - 1] {
                continue;
            } else {
                let mut left_iter = x + 1;
                let mut right_iter = nums.len() - 1;
                while left_iter < right_iter {
                    let sum = nums[x] + nums[left_iter] + nums[right_iter];
                    match sum.cmp(&0) {
                        std::cmp::Ordering::Equal => {
                            result.push(vec![nums[x], nums[left_iter], nums[right_iter]]);
                            while left_iter + 1 < right_iter
                                && nums[left_iter] == nums[left_iter + 1]
                            {
                                left_iter += 1;
                            }
                            while left_iter < right_iter - 1
                                && nums[right_iter] == nums[right_iter - 1]
                            {
                                right_iter -= 1;
                            }
                            left_iter += 1;
                            right_iter -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            left_iter += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            right_iter -= 1;
                        }
                    }
                }
            }
        }
        result
    }
}

#[test]
pub fn problem_15_test() {
    for x in Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]) {
        assert_eq!(x.iter().sum::<i32>(), 0);
    }
}
