use crate::Solution;

// TODO ::  There are definitely better ways of doing this...
//          The trick is to understand the progression of the number.
//          Its very tricky tho...

impl Solution {
    pub fn convert<T>(s: T, n: i32) -> String
    where
        T: Into<String>,
    {
        if n == 1 {
            return s.into();
        }
        // Variable declarations
        let s = s.into();
        let n = n as usize;

        let mut result: Vec<String> = vec![String::with_capacity(s.len() / n); n as usize];
        let mut counter: i32 = 0;
        let mut delta: i32 = 0;
        s.chars().for_each(|x| {
            if counter == 0 {
                delta = 1;
            } else if counter == (n - 1) as i32 {
                delta = -1;
            }
            match result.get_mut(counter as usize) {
                Some(some) => {
                    (*some).push(x);
                    counter += delta;
                }
                None => {
                    panic!(
                        "index out of bound counter:{} size:{}",
                        counter,
                        result.len()
                    );
                }
            }
        });
        result.iter().fold(String::new(), |acc, x| acc + x)
    }
}

#[test]
fn problem_6_test() {
    // assert_eq!(Solution::convert("PAYPALISHIRING", 3), "PAHNAPLSIIGYIR");
    // assert_eq!(Solution::convert("PAYPALISHIRING", 4), "PINALSIGYAHRPI");
    assert_eq!(Solution::convert("AB", 1), "AB");
}
