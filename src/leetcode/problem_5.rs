use crate::Solution;

impl Solution {
    // TODO ::  I am pretty sure this abomination can be improved by a lot...
    // TODO ::  Needs to improve the performance because we are failing on large palindromes
    //          The problem is mainly due to the fact that we are mostly doing repetitive works on them.
    // TODO ::  You can introduce an exit early function because say if the current largest palindrome is N long.
    //          Then the rightward iteration must be at least N long as well.
    pub fn longest_palindrome<T>(s: T) -> String
    where
        T: Into<String>,
    {
        let s = s.into();
        match s.len() {
            0 => String::new(),
            _ => {
                s.chars()
                    .enumerate()
                    .map(|(current_index, current_character)| {
                        let mut current_palindromes: Vec<String> = Vec::new();
                        {
                            // If single center i.e. ....bab.....
                            let leftward_iteration = (0..current_index)
                                .map(|index| {
                                    let ch = s.chars().enumerate().nth(index).unwrap();
                                    ch
                                })
                                .rev();
                            let rightward_iteration = (current_index..s.len())
                                .map(|index| {
                                    let ch = s.chars().enumerate().nth(index).unwrap();
                                    ch
                                })
                                // NOTE :: This is 1 of 2 places where this code block is different than the one below...
                                .filter(|&x| x.0 != current_index); // Filters out the current index from rightward iteration
                            let mut current_palindrome = leftward_iteration
                                .zip(rightward_iteration)
                                .take_while(|(left, right)| right.1 == left.1)
                                .map(|(left, right)| vec![left, right])
                                .flatten()
                                .collect::<Vec<(usize, char)>>();
                            // NOTE :: This is the other 1 of 2 places where this code block is different than the one below...
                            current_palindrome.push((current_index, current_character)); // Push the single character in the middle
                            current_palindrome.sort_by(|&left, &right| left.0.cmp(&right.0));
                            let palindrome_single =
                                current_palindrome
                                    .iter()
                                    .fold(String::new(), |mut acc, &x| {
                                        acc.push(x.1);
                                        acc
                                    });
                            current_palindromes.push(palindrome_single);
                        }
                        {
                            // If dual center i.e. ...aa....
                            let leftward_iteration = (0..current_index)
                                .map(|index| {
                                    let ch = s.chars().enumerate().nth(index).unwrap();
                                    ch
                                })
                                .rev();
                            let rightward_iteration = (current_index..s.len()).map(|index| {
                                let ch = s.chars().enumerate().nth(index).unwrap();
                                ch
                            });
                            let mut current_palindrome = leftward_iteration
                                .zip(rightward_iteration)
                                .take_while(|(left, right)| right.1 == left.1)
                                .map(|(left, right)| vec![left, right])
                                .flatten()
                                .collect::<Vec<(usize, char)>>();
                            current_palindrome.sort_by(|&left, &right| left.0.cmp(&right.0));
                            let palindrome_single =
                                current_palindrome
                                    .iter()
                                    .fold(String::new(), |mut acc, &x| {
                                        acc.push(x.1);
                                        acc
                                    });
                            current_palindromes.push(palindrome_single);
                        }
                        current_palindromes
                    })
                    .flatten()
                    .filter(|x| !x.is_empty())
                    .max_by(|left, right| left.len().cmp(&right.len()))
                    .unwrap()
            }
        }
    }
}

#[test]
fn problem_5_test() {
    assert_eq!(Solution::longest_palindrome("babad").len(), 3);
    assert_eq!(Solution::longest_palindrome("cbbd").len(), 2);
    assert_eq!(Solution::longest_palindrome("ccc").len(), 3);
    assert_eq!(Solution::longest_palindrome("aaaa").len(), 4);
    assert_eq!(Solution::longest_palindrome("").len(), 0);
    assert_eq!(Solution::longest_palindrome("b").len(), 1);
    //  TODO    ::  Time limit exceeded for this test case
    // assert_eq!(Solution::longest_palindrome("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffgggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"
    // ).len(), 494);
}
