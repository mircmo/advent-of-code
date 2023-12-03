use std::{cmp, fs};

enum Cell {
    Number(char),
    Symbol,
    None,
}

fn calculate_sum_by_neighbours(
    field: &Vec<Vec<Cell>>,
    x: usize,
    y: usize,
    current_num: &Vec<char>,
) -> u32 {
    let (y_max, x_max) = (field.len() - 1, field[0].len() - 1);

    for i in cmp::max(0, x as i32 - current_num.len() as i32)..=cmp::min(x_max as i32, x as i32 + 1)
    {
        for j in cmp::max(0, y as i32 - 1)..=(cmp::min(y_max as i32, y as i32 + 1)) {
            if let Cell::Symbol = field[j as usize][i as usize] {
                return current_num.iter().collect::<String>().parse().unwrap();
            }
        }
    }
    return 0;
}

fn parse(data: &str) -> Vec<Vec<Cell>> {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => Cell::None,
                    _ if char.is_digit(10) => Cell::Number(char),
                    _ => Cell::Symbol,
                })
                .collect::<Vec<Cell>>()
        })
        .collect()
}

pub(crate) fn first() -> u32 {
    let data = fs::read_to_string("./src/data/day3.txt").expect("error opening file");

    let field = parse(&data);

    let mut sum = 0;
    for (y, row) in field.iter().enumerate() {
        let mut current_num: Vec<char> = Vec::new();

        for (x, cell) in row.iter().enumerate() {
            if let Cell::Number(digit) = cell {
                current_num.push(*digit);
            } else if !current_num.is_empty() {
                // this means the number was complete at the previous item
                sum += calculate_sum_by_neighbours(&field, x - 1, y, &current_num);
                current_num.clear();
            }
        }
        // at the end of each row, finish the current number
        if !current_num.is_empty() {
            sum += calculate_sum_by_neighbours(&field, row.len() - 1, y, &current_num);
            current_num.clear()
        }
    }
    return sum;
}
