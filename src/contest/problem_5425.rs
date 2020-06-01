use crate::Solution;

impl Solution {
    pub fn max_area_5425(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
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
        (((max_horizontal_cut_adjacent_difference as u64)
            * (max_vertical_cut_adjacent_difference as u64))
            % (u64::pow(10, 9) + 7)) as i32
    }
}

#[test]
pub fn test_contest_5425() {
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
pub fn test_contest_5425_2() {
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
pub fn test_contest_5425_3() {
    let input_1 = 5;
    let input_2 = 4;
    let input_3 = vec![3, 1];
    let input_4 = vec![1];
    assert_eq!(
        870431178,
        Solution::max_area_5425(input_1, input_2, input_3, input_4)
    );
}
