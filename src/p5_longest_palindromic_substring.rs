#[test]
fn case_1() {
    assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
}

#[test]
fn case_2() {
    assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
}

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if let 0 | 1 = s.len() {
            return s;
        }

        let vec: Vec<_> = s.chars().collect();

        for length in (1..s.len() + 1).rev() {
            let places = s.len() - length + 1;

            for left in 0..places {
                if Solution::is_palindrome(&vec, left, length) {
                    return s[left..left + length].to_owned();
                }
            }
        }

        String::new()
    }

    fn is_palindrome(vec: &[char], start: usize, length: usize) -> bool {
        let end = start + length - 1;
        for i in 0..(length / 2) {
            if vec[start + i] != vec[end - i] {
                return false;
            }
        }

        true
    }
}
