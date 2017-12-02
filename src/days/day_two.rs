use super::helper;

pub fn part_one() -> u32 {
    let input = helper::input::into_string("./days/day_02.txt");
    let mut total: u32 = 0;
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect();
    for row in input.iter() {
        total += row.iter().max().unwrap() - row.iter().min().unwrap();
    }
    total
}

pub fn part_two() -> u32 {
    let input = helper::input::into_string("./days/day_02.txt");
    let mut total: u32 = 0;
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect();
    //boring old iterable version with nested for loops :(
    for row in input.iter() {
        for num in row.iter() {
            for num2 in row.iter() {
                if num % num2 == 0 && num != num2 {
                    total += num / num2;
                }
            }
        }
    }
    total
}
