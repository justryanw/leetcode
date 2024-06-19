#[test]
fn case_1() {
    let input = vec!["777", "7", "77", "77"]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
    assert_eq!(Solution::num_of_pairs(input, "7777".to_owned()), 4)
}

#[test]
fn case_2() {
    let input = vec!["123", "4", "12", "34"]
        .iter()
        .map(|&s| s.to_owned())
        .collect();
    assert_eq!(Solution::num_of_pairs(input, "1234".to_owned()), 2)
}

#[test]
fn case_3() {
    let input = vec!["1", "1", "1"].iter().map(|&s| s.to_owned()).collect();
    assert_eq!(Solution::num_of_pairs(input, "11".to_owned()), 6)
}

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_of_pairs(nums: Vec<String>, target: String) -> i32 {
        let count_map: HashMap<String, u32> = nums.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x.to_string()).or_insert(0) += 1;
            acc
        });

        let mut pairs = 0;

        for i in 0..target.len() {
            let start = &target[..i];

            let Some(start_count) = count_map.get(start) else {
                continue;
            };

            let end = &target[i..];

            let Some(end_count) = count_map.get(end) else {
                continue;
            };

            pairs += start_count * end_count;

            if start == end {
                pairs -= start_count;
            }
        }

        pairs as i32
    }
}
