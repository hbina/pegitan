use crate::Solution;

impl Solution {
    //  NOTE    ::  Naive implementation, simply collect the permutations as we go.
    pub fn can_partition_naive(nums: Vec<i32>) -> bool {
        let mut sum: i32 = nums.iter().sum();
        let mut result: Vec<(i32, i32)> = Vec::new();
        nums.iter().for_each(|&x| match result.len() {
            0 => {
                sum -= x;
                result.push((x, 0))
            }
            _ => {
                let right_result = result
                    .iter()
                    .map(|&(left, right)| (left, right + x))
                    .collect::<Vec<(i32, i32)>>();
                let left_result = result
                    .iter()
                    .map(|&(left, right)| (left + x, right))
                    .collect::<Vec<(i32, i32)>>();
                result.clear();
                result.extend(left_result);
                result.extend(right_result);
            }
        });
        result
            .iter()
            .filter(|&&(left, right)| left == right)
            .collect::<Vec<_>>()
            .len()
            != 0
    }

    //  NOTE    ::  Checks if no possible combination exists. We do this by taking the absolute
    //              difference and checking if its more than the sum of the whats left.
    pub fn can_partition_whats_left(nums: Vec<i32>) -> bool {
        let mut sum: i32 = nums.iter().sum();
        let mut result: Vec<(i32, i32)> = Vec::new();
        nums.iter().for_each(|&x| match result.len() {
            0 => {
                sum -= x;
                result.push((x, 0))
            }
            _ => {
                let new_result = result
                    .iter()
                    .filter(|&&(left, right)| match left.cmp(&right) {
                        std::cmp::Ordering::Greater => left - right <= sum,
                        _ => right - left <= sum,
                    })
                    .map(|&(left, right)| vec![(left + x, right), (left, right + x)])
                    .flatten()
                    .collect::<Vec<(i32, i32)>>();
                result = new_result;
                sum -= x;
            }
        });
        result
            .iter()
            .filter(|&&(left, right)| left == right)
            .collect::<Vec<_>>()
            .len()
            != 0
    }

    //  NOTE    ::  This function cannot produce false positive!
    pub fn can_partition_sort_and_try(mut nums: Vec<i32>) -> bool {
        nums.sort_by(|x, y| y.cmp(x));
        nums.iter().fold(0, |acc, &x| match acc > 0 {
            true => acc - x,
            false => acc + x,
        }) == 0
    }

    //  NOTE    ::  This implementation is horse-crap :3
    pub fn can_partition_odd_even_counter(nums: Vec<i32>) -> bool {
        // Must sanitize
        let sum: i32 = nums.iter().sum();
        let sanitize = nums.iter().fold(true, |acc, &x| acc && (sum - x) >= x);
        if !sanitize {
            return false;
        }
        // Perform actual algorithm...
        let (mut twos, mut ones) = (0, 0);
        nums.iter().for_each(|&x| match x % 2 == 0 {
            true => {
                twos += x / 2;
            }
            false => {
                ones += 1;
                twos += (x - 1) / 2;
            }
        });
        if twos % 2 != 0 && ones >= 2 {
            twos -= 1;
            ones -= 2;
        }
        twos % 2 == 0 && ones % 2 == 0
    }

    pub fn can_partition_dp_top_down(
        nums: &Vec<i32>,
        start_from: usize,
        whats_left: i32,
        goal_amount: i32,
    ) -> bool {
        // Exit if we have ran through every possible integers
        if start_from >= nums.len() {
            return false;

        // Exit if the goal amount is larger than whats left in the array, meaning even if we
        // consider every element afterwards, we will still fail
        } else if goal_amount > whats_left {
            return false;

        // Exit with true if considering this value means that we found a summation that yields
        // whats left of sum/2 after all the considerations made beforehand.
        } else if goal_amount == whats_left {
            return true;
        } else {
            // Else, generate 2 recursive calls that:
            // 1. considers this value (if possible)
            // 2. does not consider this value
            // And returning the OR or those calls.
            let next_start_from = start_from + 1;
            let current_value = nums[start_from];
            let next_whats_left = whats_left - current_value;
            match goal_amount.cmp(&current_value) {
                std::cmp::Ordering::Greater => {
                    return Solution::can_partition_dp_top_down(
                        &nums,
                        next_start_from,
                        next_whats_left,
                        goal_amount - current_value,
                    ) || Solution::can_partition_dp_top_down(
                        &nums,
                        next_start_from,
                        next_whats_left,
                        goal_amount,
                    );
                }
                std::cmp::Ordering::Equal => {
                    return true;
                }
                std::cmp::Ordering::Less => {
                    return Solution::can_partition_dp_top_down(
                        &nums,
                        next_start_from,
                        next_whats_left,
                        goal_amount,
                    );
                }
            }
        }
    }

    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            false
        } else {
            nums.sort_by(|x, y| y.cmp(x));
            Solution::can_partition_dp_top_down(&nums, 0, sum, sum / 2)
        }
    }
}

#[test]
fn problem_416_test() {
    assert_eq!(Solution::can_partition(vec![1, 5, 11, 5]), true);
    assert_eq!(Solution::can_partition(vec![5, 3, 4, 4]), true);
    assert_eq!(Solution::can_partition(vec![3, 3, 1, 94]), false);
    assert_eq!(Solution::can_partition(vec![5, 3, 1, 4]), false);
    assert_eq!(Solution::can_partition(vec![3, 3, 3, 4, 5]), true);
    assert_eq!(Solution::can_partition(vec![1, 2, 5]), false);
    assert_eq!(
        Solution::can_partition(vec![
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100,
            100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100, 100
        ]),
        true
    );
    assert_eq!(Solution::can_partition(vec![2, 2, 3, 5]), false);
    assert_eq!(
        Solution::can_partition(vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 100
        ]),
        false
    );
}

#[test]
pub fn problem_416_failing() {}
