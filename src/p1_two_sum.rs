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

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        vec![]
    }

    // pub fn print_add(a: i32, b: i32) {
    //     let sum = a + b;
    //     println!("{sum}");
    // }
}
