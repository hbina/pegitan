use crate::Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // If we don't even have more than 3 elements, we just return their sum
        if nums.len() <= 3 {
            return nums.iter().sum();
        }

        nums.sort();
        let (mut result, mut global_diff) = (
            nums[0] + nums[1] + nums[2],
            target - (nums[0] + nums[1] + nums[2]),
        );
        // println!("result:{} global_diff:{}", result, global_diff);
        for x in 0..nums.len() {
            if x > 0 && nums[x] == nums[x - 1] {
                continue;
            } else {
                let mut left_iter = x + 1;
                let mut right_iter = nums.len() - 1;
                while left_iter < right_iter {
                    // Pre-calculations
                    let sum = nums[x] + nums[left_iter] + nums[right_iter];
                    let diff = target - sum;
                    // println!("sum:{} diff:{}", sum, diff);

                    // If the local diff is smaller than global diff, replace it
                    if diff.abs() < global_diff.abs() {
                        global_diff = diff;
                        result = sum;
                        // println!("new global_diff:{} result:{}", global_diff, result);
                    }

                    // Depending on the local summation, we move the left/right pointer accordingly.
                    match sum.cmp(&target) {
                        std::cmp::Ordering::Greater => {
                            right_iter -= 1;
                        }
                        std::cmp::Ordering::Less => {
                            left_iter += 1;
                        }
                        std::cmp::Ordering::Equal => {
                            return sum;
                        }
                    }
                }
            }
        }
        result
    }
}

#[test]
pub fn problem_16_test() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    assert_eq!(Solution::three_sum_closest(vec![0, 2, 1, -3], 1), 0);
    assert_eq!(Solution::three_sum_closest(vec![-1, 0, 1, 1, 55], 3), 2);
    assert_eq!(Solution::three_sum_closest(vec![-3, -2, -5, 3, -4], -1), -2);
    assert_eq!(Solution::three_sum_closest(vec![0, -4, 1, -5], 0), -3);
}
