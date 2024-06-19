#[test]
fn case_1() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn case_2() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}

pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let lenght = height.len() as i32;
        let iter = height.clone().into_iter();
        let heighest = iter.clone().max().unwrap();

        let mut water = 0;

        for scan_height in 1..=heighest {
            let (min, max) = iter.clone().enumerate().fold(
                (lenght, 0),
                |(mut min, mut max), (index, terrain_height)| {
                    let index = index as i32;

                    if terrain_height >= scan_height {
                        if index < min {
                            min = index;
                        }
                        if index > max {
                            max = index;
                        }
                    }

                    (min, max)
                },
            );

            for index in min..max {
                if height[index as usize] < scan_height {
                    water += 1;
                }
            }
        }

        water
    }
}
