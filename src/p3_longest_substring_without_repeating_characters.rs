#[test]
fn case_1() {
    assert_eq!(3, Solution::length_of_longest_substring("abcabcbb".to_string()));
}

#[test]
fn case_2() {
    assert_eq!(1, Solution::length_of_longest_substring("bbbbb".to_string()));
}

#[test]
fn case_3() {
    assert_eq!(3, Solution::length_of_longest_substring("pwwkew".to_string()));
}

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        0
    }
}
