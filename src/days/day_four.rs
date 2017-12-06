use super::helper;
use std::collections::HashSet;

pub fn part_one() -> usize {
    let input = helper::input::into_string("./days/day_four.txt");
    // Need to use a new variable because of the rust type system >:(
    let password_list: Vec<&str> = input.lines().collect();
    let mut valid_passwords: usize = password_list.len();
    let mut password_set = HashSet::new();
    for password in password_list.iter() {
        for word in password.split(' ') {
            if !password_set.insert(word) {
                valid_passwords -= 1;
                break;
            }
        }
        password_set.clear();
    }
    valid_passwords
}
