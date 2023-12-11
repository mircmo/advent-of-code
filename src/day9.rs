use std::fs;

pub(crate) fn first() -> i32 {
    fs::read_to_string("./src/data/day9.txt")
        .expect("error opening file")
        .lines()
        .map(|line| {
            let mut nums: Vec<i32> = line
                .split_whitespace()
                .filter_map(|str| str.parse().ok())
                .collect();

            let mut top_right_value = 0i32;
            return loop {
                if nums.iter().all(|i| i == &0) {
                    break top_right_value;
                }
                for i in 0..nums.len() - 1 {
                    nums[i] = nums[i + 1] - nums[i];
                }
                top_right_value += nums.pop().unwrap()
            };
        })
        .sum()
}
