use std::{cmp, collections::VecDeque, fs};

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

            (count > 0).then(|| 2u32.pow(count - 1))
        })
        .sum()
}

pub(crate) fn second() -> u32 {
    let data = fs::read_to_string("./src/data/day4.txt").expect("error opening file");

    data.lines()
        .fold(
            (0, VecDeque::new() as VecDeque<u32>),
            |(sum, mut copies_tracker), line| {
                let sanitized_line = line.replace("  ", " ");
                let (_, numbers) = sanitized_line.split_once(": ").unwrap();
                let (winning, mine) = numbers.split_once(" | ").unwrap();

                let winning_nums: Vec<&str> = winning.split_whitespace().collect();
                let my_nums: Vec<&str> = mine.split_whitespace().collect();

                let instances_of_current_card = copies_tracker.pop_front().unwrap_or(0) + 1;

                let new_copies: VecDeque<u32> = my_nums
                    .into_iter()
                    .filter_map(|my_num| {
                        winning_nums
                            .contains(&my_num)
                            .then_some(instances_of_current_card)
                    })
                    .collect();

                if new_copies.len() > copies_tracker.len() {
                    copies_tracker.resize(new_copies.len(), 0)
                }
                copies_tracker
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, item)| *item += new_copies.get(i).unwrap_or(&0));

                return (sum + instances_of_current_card, copies_tracker);
            },
        )
        .0
}
