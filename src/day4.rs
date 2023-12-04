use std::fs;

pub(crate) fn first() -> u32 {
    let data = fs::read_to_string("./src/data/day4.txt").expect("error opening file");

    data.lines()
        .filter_map(|line| {
            let sanitized_line = line.replace("   ", " ").replace("  ", " ");
            let (_, numbers) = sanitized_line.split_once(": ").unwrap();
            let (winning, mine) = numbers.split_once(" | ").unwrap();

            let winning_nums: Vec<&str> = winning.split_whitespace().collect();
            let my_nums: Vec<&str> = mine.split_whitespace().collect();

            let count = my_nums
                .into_iter()
                .filter(|my_num| winning_nums.contains(my_num))
                .count() as u32;

            (count > 0).then(|| 2_u32.pow(count - 1))
        })
        .sum()
}
