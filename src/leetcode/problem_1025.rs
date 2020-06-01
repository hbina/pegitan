use crate::Solution;

impl Solution {
    //  NOTE    ::  A brief explanation on why this "works" is available at:
    //              https://medium.com/@hanif.ariffin.4326/intuition-on-leetcodes-problem-1025-divisor-game-daf00e77d0ce
    pub fn divisor_game(n: i32) -> bool {
        n % 2 == 0
    }

    //  TODO    ::  This can be improved further by considering the
    //              distribution of the divisors of `N`.
    //              In particular, if `N` is even, we know that the largest divisor is `N/2`.
    //              Furthermore, if `N` is odd, then the smallest divisor must be `< sqrt(N)`
    //  TODO    ::  Another improvement that can be made is to notice that we only consider the divisor of a number N
    pub fn divisor_game_recursive(n: i32) -> bool {
        match n {
            1 => false,
            2 => true,
            3 => false,
            4 => true,
            _ => {
                (1..=n / 2)
                    .collect::<Vec<_>>()
                    .iter()
                    .filter(|&&x| n % x == 0)
                    .take_while(|&&x| !Solution::divisor_game_recursive(n - x))
                    .collect::<Vec<_>>()
                    .len()
                    != 0
            }
        }
    }
}

#[test]
fn problem_1025_test() {
    assert_eq!(Solution::divisor_game(1), false);
    assert_eq!(Solution::divisor_game(2), true);
    assert_eq!(Solution::divisor_game(3), false);
    assert_eq!(Solution::divisor_game(4), true);
    assert_eq!(Solution::divisor_game(5), false);

    assert_eq!(Solution::divisor_game_recursive(1), false);
    assert_eq!(Solution::divisor_game_recursive(2), true);
    assert_eq!(Solution::divisor_game_recursive(3), false);
    assert_eq!(Solution::divisor_game_recursive(4), true);
    assert_eq!(Solution::divisor_game_recursive(5), false);

    (5..20).for_each(|x| {
        assert_eq!(
            Solution::divisor_game_recursive(x),
            Solution::divisor_game(x)
        );
    });
}
