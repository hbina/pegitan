use crate::Solution;

impl Solution {
    pub fn simplify_path<T>(path: T) -> String
    where
        T: Into<String>,
    {
        let mut stack: Vec<&str> = Vec::new();
        let path = path.into();
        path.split('/').for_each(|c: &str| {
            if c == ".." {
                if stack.len() > 0 {
                    stack.pop();
                }
            } else if c != "." && c.len() > 0 {
                stack.push(c);
            }
        });
        if stack.is_empty() {
            String::from("/")
        } else {
            stack
                .into_iter()
                .filter(|c| !c.is_empty())
                .fold(String::new(), |mut acc, x| {
                    acc.push_str("/");
                    acc.push_str(x);
                    acc
                })
        }
    }
}

#[test]
fn problem_71_test() {
    assert_eq!(Solution::simplify_path("/home/"), "/home");
    assert_eq!(Solution::simplify_path("/../"), "/");
    assert_eq!(Solution::simplify_path("/home//foo/"), "/home/foo");
    assert_eq!(Solution::simplify_path("/a/./b/../../c/"), "/c");
    assert_eq!(Solution::simplify_path("/a/../../b/../c//.//"), "/c");
    assert_eq!(Solution::simplify_path("/a//b////c/d//././/.."), "/a/b/c");
}
