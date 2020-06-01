// Sorting Algorithm Implementations
// Some of the algorithms provided below can be found in CLRS and will be mentioned as such.

// NAME ::  Insertion sort
// COMP ::  O(N^2)
// MEMC ::  O(1)
// See page 18 of CLRS.
// NOTE ::  This is a bit different from the pseudocode provided in CLRS.
//          The difference is mainly due to the fact that Rust will hard crash upon integer overflow.
//          The problem is that Rust indexing uses `usize` type which is unsigned and the algorithm for
//          the second iteration in the algorithm rely on the value being negative (-1).
//          Not sure how to proceed with the actual algorithm, so we just modify it a bit here.
// TODO ::  Make this a generic function that accepts anything iterable
pub fn insertion_sort<T>(input: &mut Vec<T>) -> &mut Vec<T>
where
    T: Ord + Copy + Clone,
{
    for i in 1..input.len() {
        for j in (1..i + 1).rev() {
            if input[j - 1] <= input[j] {
                break;
            }
            let tmp = input[j - 1];
            input[j - 1] = input[j];
            input[j] = tmp;
        }
    }
    input
}

#[test]
fn test() {
    assert_eq!(
        insertion_sort(&mut vec![5, 2, 4, 6, 1, 3]),
        &mut vec![1, 2, 3, 4, 5, 6]
    );
}
