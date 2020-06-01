// TODO ::  Make this generic that accepts anything that can be turned into a String
#[allow(dead_code)]
fn is_palindrome(word: String) -> bool {
    let result: usize = word
        .chars()
        .rev()
        .enumerate()
        .zip(word.chars().enumerate())
        .take_while(|((forward_index, c1), (reverse_index, c2))| {
            c1 == c2 && forward_index + reverse_index < word.len()
        })
        .map(|((_, ch), _)| ch.len_utf8())
        .sum();
    match word.len() % 2 == 0 {
        true => result == word.len() / 2,
        false => result == (word.len() + 1) / 2,
    }
}

#[test]
fn test() {
    assert_eq!(is_palindrome(String::from("abcdcba")), true);
    assert_eq!(is_palindrome(String::from("abccba")), true);
    assert_eq!(is_palindrome(String::from("aa")), true);
    assert_eq!(is_palindrome(String::from("abb")), false);
}
