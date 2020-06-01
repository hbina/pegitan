use crate::Solution;

impl Solution {
    pub fn total_fruit(_: Vec<i32>) -> i32 {
        // TODO ::  Find the first 2 couple.
        //          We can actually exit early based on this algorithm.
        //          For now we will just use HashMap
        // TODO ::  Calibrating the global maximum stuff can be extracted into a closure...
        0
    }
}

#[test]
fn problem_904_failing() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    assert_eq!(
        Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
        5
    );
    assert_eq!(Solution::total_fruit(vec![1, 0, 1, 4, 1, 4, 1, 2, 3]), 5);
}
