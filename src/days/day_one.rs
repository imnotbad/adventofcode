use super::helper;

pub fn part_one() -> u32 {
    let mut sum: u32 = 0;
    let input = helper::input::into_string("./days/day_one.txt");
    // turn input into digits
    let numbers: Vec<u32> = input
        .chars()
        .map(|num| {
            num.to_digit(10).expect("numbers should be ascii") as u32
        })
        .collect();
    for (i, num) in numbers.iter().enumerate() {
        if i + 1 == numbers.len() {
            break;
        } else if *num == numbers[i + 1] {
            sum += num;
        }
    }
    // handle last number matching first number
    if numbers[0] == numbers[numbers.len() - 1] {
        sum += numbers[0]
    }
    sum
}

pub fn part_two() -> u32 {
    let mut sum: u32 = 0;
    let input = helper::input::into_string("./days/day_one_part2.txt");
    // turn input into digits
    let numbers: Vec<u32> = input
        .chars()
        .map(|num| {
            num.to_digit(10).expect("numbers should be ascii") as u32
        })
        .collect();
    // create a zipped iterator containing middle values
    let numbers_mid = numbers.len()/2;
    for (num, mid_num) in numbers.iter().zip(numbers.iter().skip(numbers_mid)) {
        if num == mid_num {
            sum += num * 2;
        }
    }
sum
}

