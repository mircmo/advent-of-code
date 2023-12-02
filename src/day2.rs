use std::{cmp, collections::HashMap, fs};

pub(crate) fn first() -> i32 {
    let color_limits = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    fs::read_to_string("./src/data/day2.txt")
        .expect("error opening file")
        .lines()
        .filter_map(|line| {
            let mut x = line.split(": ");
            let id = x
                .next()
                .unwrap()
                .split("Game ")
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap();

            let has_error = x.next().unwrap().split("; ").any(|turn| {
                turn.split(", ").any(|roll| {
                    let mut color_and_amount = roll.split(" ");
                    let amount = color_and_amount
                        .next()
                        .unwrap()
                        .trim()
                        .parse::<i32>()
                        .unwrap();
                    let color = color_and_amount.next().unwrap();
                    let limit = color_limits.get(&color).unwrap();

                    return amount > *limit;
                })
            });

            return (!has_error).then_some(id);
        })
        .sum()
}

pub(crate) fn second() -> i32 {
    fs::read_to_string("./src/data/day2.txt")
        .expect("error opening file")
        .lines()
        .map(|line| {
            let (mut r, mut g, mut b) = (0, 0, 0);

            line.split(": ")
                .last()
                .unwrap()
                .split("; ")
                .for_each(|turn| {
                    turn.split(", ").for_each(|roll| {
                        let mut color_and_amount = roll.split(" ");
                        let amount = color_and_amount
                            .next()
                            .unwrap()
                            .trim()
                            .parse::<i32>()
                            .unwrap();
                        let color = color_and_amount.next().unwrap();
                        match color {
                            "red" => r = cmp::max(amount, r),
                            "blue" => b = cmp::max(amount, b),
                            "green" => g = cmp::max(amount, g),
                            _ => panic!("impossible!"),
                        };
                        // var += amount;
                    })
                });

            return r * g * b;
        })
        .sum()
}
