use std::fs;

pub(crate) fn first() -> i64 {
    let data = fs::read_to_string("./src/data/day5.txt").expect("error opening file");

    let lines = &mut data.lines();
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap() 
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    let rest = lines.skip(1).collect::<Vec<&str>>().join("\n");

    let maps: Vec<&str> = rest.split("\n\n").collect();

    maps.into_iter()
        .fold(seeds, |mut acc, map| {
            let previous_state = acc.clone();
            map.lines().skip(1).for_each(|line| {
                let nums: Vec<i64> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect();
                let (dest_start, source_start, len) = (nums[0], nums[1], nums[2]);
                let diff = dest_start - source_start;

                previous_state.iter().enumerate().for_each(|(i, num)| {
                    if *num >= source_start && *num < (source_start + len) {
                        acc[i] = num + diff
                    }
                })
            });
            acc
        })
        .iter()
        .min()
        .unwrap()
        .clone()
}
