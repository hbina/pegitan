//  NOTE    ::  From Lecture 1 : Algorithmic Thinking
//              This function accepts a vector T that satisfy comparison trait and returns the index
//              to that element.
//  TODO    ::  Actually turn this into a generic algorithm that accepts anything that can be compared with.
//              If possible, allow it to accept anything that is iterable!
//  TODO    ::  Fix copying the values multiple times...
pub fn peak_finder<T>(a: Vec<T>) -> usize
where
    T: PartialOrd + Clone,
{
    match a.len() {
        0 => {
            panic!("please provide a non empty vector!");
        }
        1 => 0,
        2 => {
            if a[0] > a[1] {
                0
            } else {
                1
            }
        }
        _ => {
            let test_index = a.len() / 2;
            if a[test_index] < a[test_index - 1] {
                peak_finder(Vec::from(&a[0..=test_index]))
            } else if a[test_index] < a[test_index + 1] {
                test_index + peak_finder(Vec::from(&a[test_index..a.len()]))
            } else {
                test_index
            }
        }
    }
}

#[test]
fn test() {
    assert_eq!(peak_finder(vec![1, 2, 3, 4, 5, 2, 1]), 4);
    assert_eq!(peak_finder(vec![1, 8, 7, 6, 5, 4, 1]), 1);
    assert_eq!(peak_finder(vec![1, 2]), 1);
    assert_eq!(peak_finder(vec![3, 2]), 0);
    assert_eq!(peak_finder(vec![1, 2, 3]), 2);
    assert_eq!(peak_finder(vec![1, 2, 1]), 1);
    assert_eq!(peak_finder(vec![1, 2, 3, 3, 3, 2, 1]), 3);
}
