use super::helper;

pub fn part_one() -> u32 {
    let mut input: Vec<i32> = helper::input::into_string("./days/day_five.txt")
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut steps: u32 = 0;
    let mut current_index: i32 = 0;
    loop {
        if let Some(instruction) = input.clone().get(current_index as usize) {
            steps += 1;
            input[current_index as usize] += 1;
            current_index += instruction;
        } else {
            break;
        }
    }
    steps
}

pub fn part_two() -> u32 {
    let mut input: Vec<i32> = helper::input::into_string("./days/day_five.txt")
        .split('\n')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut steps: u32 = 0;
    let mut current_index: i32 = 0;
    loop {
        if let Some(instruction) = input.clone().get(current_index as usize) {
            steps += 1;
            if *instruction >= 3 {
                input[current_index as usize] -= 1;
            } else {
                input[current_index as usize] += 1;
            }
            current_index += instruction;
        } else {
            break;
        }
    }
    steps
}
