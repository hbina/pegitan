use crate::Solution;

impl Solution {
    //  TODO    ::  This can be refactored to use anonymous function that accepts an iterator that
    //              is reversible and accepts some additional parameter to determine the correcting factor.
    //              Or at least we have to rethink how to transform this...
    pub fn max_area_naive(height: Vec<i32>) -> i32 {
        let mut max = 0;
        for x in height.iter().enumerate() {
            for y in height.iter().enumerate().filter(|&y| y.0 > x.0) {
                max = std::cmp::max(max, (y.0 - x.0) as i32 * std::cmp::min(*x.1, *y.1));
            }
        }
        max
    }

    pub fn max_area_single_pass(height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right = height.len() - 1;
        let mut max = 0;
        while left < right {
            let left_height = height[left];
            let right_height = height[right];
            let width = (right - left) as i32;
            let area = std::cmp::min(left_height, right_height) * width;
            max = std::cmp::max(area, max);
            match left_height.cmp(&right_height) {
                std::cmp::Ordering::Greater => right -= 1,
                _ => left += 1,
            }
        }
        max
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        Solution::max_area_naive(height)
    }
}

#[test]
pub fn problem_11_test() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    assert_eq!(Solution::max_area(vec![1, 2]), 1);
    assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
}
