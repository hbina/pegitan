use crate::Solution;

impl Solution {
    pub fn coin_change_brute_force(coins: Vec<i32>, amount: i32) -> i32 {
        Solution::coin_change_brute_force_recursion(0, &coins, amount)
    }
    //  NOTE    ::  Current solution involves modding the value until the value is 0.
    //              Otherwise, the must be no solution available. However, I am not entirely
    //              confident that this is correct.
    //              This solution is apparently insufficient. It assumes that each denomination is used.
    pub fn coin_change_brute_force_recursion(
        coin_idx: usize,
        coins: &Vec<i32>,
        amount: i32,
    ) -> i32 {
        println!("amount:{}", amount);
        match amount.cmp(&0) {
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => {
                if let Some(&current_coin) = coins.get(coin_idx) {
                    let max_possible_coin_count = amount / current_coin;
                    let mut current_coin_count = std::i32::MAX;
                    println!(
                        "current_coin:{} max_possible_coin_count:{}",
                        current_coin, max_possible_coin_count
                    );
                    for x in 0..=max_possible_coin_count {
                        println!(
                            "assume we pick x:{} of coin:{} when amount:{}",
                            x, current_coin, amount
                        );
                        if amount >= x * current_coin {
                            let res = Solution::coin_change_brute_force_recursion(
                                coin_idx + 1,
                                &coins,
                                amount - x * current_coin,
                            );
                            println!(
                                "the result if we pick x:{} of coin:{} is {} when amount:{}",
                                x, current_coin, res, amount
                            );
                            if res != -1 {
                                println!(
                                    "testing min of current_coin_count:{} res+x:{}",
                                    current_coin_count,
                                    res + x
                                );
                                current_coin_count = std::cmp::min(current_coin_count, res + x);
                            }
                        }
                    }
                    if current_coin_count == std::i32::MAX {
                        -1
                    } else {
                        current_coin_count
                    }
                } else {
                    println!("index:{} out of range", coin_idx);
                    -1
                }
            }
        }
    }

    // Leetcode's solution to the problem converted from Java.
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let max = amount + 1;
        let mut dp: Vec<i32> = vec![max; (amount as usize) + 1];
        dp[0] = 0;
        for i in 1..=amount {
            coins.iter().for_each(|&x| {
                if x <= i {
                    let cached_index = (i - x) as usize;
                    let current_index = i as usize;
                    dp[current_index] = std::cmp::min(dp[current_index], dp[cached_index] + 1);
                }
            });
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[test]
fn problem_322_test() {
    assert_eq!(Solution::coin_change(vec![1, 3, 5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![2, 4], 6), 2);
    assert_eq!(Solution::coin_change(vec![10, 100], 45), -1);
    assert_eq!(Solution::coin_change(vec![10, 100], 50), 5);
    assert_eq!(Solution::coin_change(vec![186, 419, 83, 408], 6249), 20);
}
