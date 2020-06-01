use crate::Solution;

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut even_counter: usize = 0;
        let mut odd_counter: usize = 0;
        let mut result = a
            .iter()
            .map(|&value| match value % 2 == 0 {
                true => {
                    even_counter += 1;
                    (value, even_counter * 2)
                }
                false => {
                    odd_counter += 1;
                    (value, odd_counter * 2 + 1)
                }
            })
            .collect::<Vec<_>>();
        result.sort_by(|left, right| left.1.cmp(&right.1));
        result.iter().map(|value| value.0).collect()
    }
}

// TODO ::  This test is not comprehensive, we need to implement a custom function to test
//          the result. One possible solution is to implement an iterator that returns alternating values.
#[test]
fn problem_922_test() {
    assert_eq!(
        Solution::sort_array_by_parity_ii(vec![4, 2, 5, 7]),
        vec![4, 5, 2, 7]
    );
}
