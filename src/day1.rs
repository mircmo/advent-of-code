use std::fs;



pub(crate) fn first() -> u32 {
    fs::read_to_string("./src/data/day1.txt")
        .expect("Error opening file")
        .lines()
        .filter_map(|line| {
            let first = line
                .chars()
                .find_map(|char| char.to_digit(10))
                .expect("no digit in line");
            let last = line
                .chars()
                .rev()
                .find_map(|char| char.to_digit(10))
                .expect("no digit in line");
            return format!("{first}{last}").parse::<u32>().ok();
        })
        .sum()
}

pub(crate) fn second() -> u32 {
    let digits_spelled = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .map(|d| d.chars().collect::<Vec<char>>());

    fs::read_to_string("./src/data/day1.txt")
        .expect("Error opening file")
        .lines()
        .filter_map(|line| {
            let mut found_digits: Vec<u32> = Vec::new();

            let chars: Vec<char> = line.chars().collect();

            chars.iter().enumerate().for_each(|(i, char)| {
                if let Some(digit) = char.to_digit(10) {
                    found_digits.push(digit);
                } else {
                    if let Some(digit) = digits_spelled
                        .iter()
                        .enumerate()
                        .find_map(|(index, word)| chars[i..].starts_with(word).then_some(index + 1))
                    {
                        found_digits.push(digit.try_into().unwrap())
                    }
                }
            });

            let fr = found_digits.first().expect("no digit found in line");
            let ls = found_digits.last().unwrap();

            return format!("{fr}{ls}").parse::<u32>().ok();
        })
        .sum()
}
