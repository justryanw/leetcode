use std::collections::HashSet;

#[test]
fn case_1() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("abcabcbb".to_string())
    );
}

#[test]
fn case_2() {
    assert_eq!(
        1,
        Solution::length_of_longest_substring("bbbbb".to_string())
    );
}

#[test]
fn case_3() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring("pwwkew".to_string())
    );
}

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let slice: Vec<char> = s.chars().collect();

        for i in (1..s.len()).rev() {
            println!("{i}");
            for substring in slice.windows(i) {
                if !Self::has_repeating_chars(substring) {
                    return i as i32;
                }
            }
        };

        1
    }

    fn has_repeating_chars(chars: &[char]) -> bool {
        let mut hashset: HashSet<char> = HashSet::new();

        for char in chars {
            if hashset.contains(char) {
                return true;
            }
            hashset.insert(*char);
        }

        false
    }
}
