use std::{collections::HashMap, fs};

pub(crate) fn first() -> u32 {
    let cards = vec![
        'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
    ];

    let data = fs::read_to_string("./src/data/day7.txt").expect("error opening file");

    let mut weighted_hands: Vec<(i32, Vec<usize>, u32)> = data
        .lines()
        .into_iter()
        .map(|line| {
            let (hand, winnings) = line.split_once(" ").unwrap();
            let mut occurences: HashMap<char, u32> = HashMap::new();

            let card_strengths: Vec<usize> = hand
                .chars()
                .filter_map(|card| {
                    *occurences.entry(card).or_insert(0) += 1;
                    cards.iter().position(|c| card == *c)
                })
                .collect();

            let mut card_counts: Vec<u32> = occurences.into_values().collect();
            card_counts.sort_by(|a, b| b.cmp(a));

            let hand_strength = match card_counts[..] {
                [5, ..] => 0,
                [4, ..] => 1,
                [3, 2, ..] => 2,
                [3, ..] => 3,
                [2, 2, ..] => 4,
                [2, ..] => 5,
                [1, 1, 1, 1, 1] => 6,
                _ => panic!("really shouldn't happen"),
            };

            return (
                hand_strength,
                card_strengths,
                winnings.parse::<u32>().unwrap(),
            );
        })
        .collect();

    // lol i can compare two vectors directly and it just works as expected :)
    weighted_hands.sort_by(|(str_a, hand_a, _), (str_b, hand_b, _)| {
        str_a.cmp(&str_b).then(hand_a.cmp(&hand_b))
    });

    weighted_hands
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, (_, _, winnings))| winnings * (i + 1) as u32)
        .sum()
}
