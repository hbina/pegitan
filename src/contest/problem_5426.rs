use crate::Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let connections = connections
            .iter()
            .map(|v| (v.first().unwrap(), v.last().unwrap()));
        let s = connections.fold(
            std::collections::HashMap::<i32, Vec<i32>>::new(),
            |mut acc, (k, v)| {
                acc.entry(*k).or_insert(Vec::new()).push(*v);
                acc
            },
        );

        let mut v = vec![0];
        let mut result = 0;
        while !v.is_empty() {
            let mut v_Copy = Vec::new();
            v.iter().for_each(|x| {
                let vv = s.get(x).unwrap();
                result += vv.len();
                v_Copy = vv.clone();
            });
            v = v_Copy;
        }
        result as i32
    }
}

#[test]
pub fn test_contest_5426() {
    let input_1 = 6;
    let input_2 = vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
    assert_eq!(3, Solution::min_reorder(input_1, input_2));
}
