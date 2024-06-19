#[test]
fn case_1() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
}

#[test]
fn case_2() {
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
}

#[test]
fn case_3() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
}

pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();

        for (index, num) in nums.iter().enumerate() {
            if let Some(other) = hashmap.get(&(target - num)) {
                return vec![*other as i32, index as i32];
            };

            hashmap.insert(num, index);
        }

        vec![]
    }
}
