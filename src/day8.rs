use std::{collections::HashMap, fs};

pub(crate) fn first() -> u32 {
    let data = fs::read_to_string("./src/data/day8.txt").expect("error opening file");
    let mut lines = data.lines();
    let instruction_set = lines.next().unwrap().chars().into_iter().cycle();

    let map: HashMap<&str, (&str, &str)> = lines
        .skip(1)
        .map(|line| {
            let (key, values) = line.split_once(" = ").unwrap();
            let parsed_values = values[1..values.len() - 1].split_once(", ").unwrap();
            return (key, parsed_values);
        })
        .collect();

    let mut current_str = "AAA";
    for (step, instruction) in (0..).zip(instruction_set) {
        if current_str == "ZZZ" {
            return step;
        }
        let (left, right) = map.get(current_str).unwrap();
        current_str = if instruction == 'L' { left } else { right };
    }
    return 0;
}
