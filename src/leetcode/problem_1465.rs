use crate::Solution;

impl Solution {
    pub fn max_area_5425(
        h: usize,
        w: usize,
        horizontal_cuts: Vec<i32>,
        vertical_cuts: Vec<i32>,
    ) -> usize {
        let mut horizontal_cuts = horizontal_cuts
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<usize>>();
        let mut vertical_cuts = vertical_cuts
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<usize>>();

        horizontal_cuts.sort();
        vertical_cuts.sort();

        //  Find largest horizontal cuts
        let horizontal_cuts = std::iter::once(0)
            .chain(horizontal_cuts)
            .chain(std::iter::once(h));
        let max_horizontal_cut_adjacent_difference = horizontal_cuts
            .clone()
            .zip(horizontal_cuts.skip(1))
            .map(|(left, right)| right - left)
            .max()
            .unwrap();

        //  Find largest vertical cuts
        let vertical_cuts = std::iter::once(0)
            .chain(vertical_cuts)
            .chain(std::iter::once(w));
        let max_vertical_cut_adjacent_difference = vertical_cuts
            .clone()
            .zip(vertical_cuts.skip(1))
            .map(|(left, right)| right - left)
            .max()
            .unwrap();
        max_horizontal_cut_adjacent_difference * max_vertical_cut_adjacent_difference
    }
}

#[test]
pub fn test_problem_1465_1() {
    let input_1 = 5;
    let input_2 = 4;
    let input_3 = vec![3];
    let input_4 = vec![3];
    assert_eq!(
        9,
        Solution::max_area_5425(input_1, input_2, input_3, input_4)
    );
}

#[test]
pub fn test_problem_1465_2() {
    let input_1 = 5;
    let input_2 = 4;
    let input_3 = vec![1, 2, 4];
    let input_4 = vec![1, 3];
    assert_eq!(
        4,
        Solution::max_area_5425(input_1, input_2, input_3, input_4)
    );
}

#[test]
pub fn test_problem_1465_3() {
    let input_1 = 5;
    let input_2 = 4;
    let input_3 = vec![3, 1];
    let input_4 = vec![1];
    assert_eq!(
        6,
        Solution::max_area_5425(input_1, input_2, input_3, input_4)
    );
}
