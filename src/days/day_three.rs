use super::helper;

pub fn part_one() -> u32 {
    // Thanks to https://github.com/teenangst for explaining how this stuff works

    // This is for consistency, you could easily hardcode the value and have compiler optimizations
    let input: f64 = helper::input::into_string("./days/day_three.txt")
        .parse()
        .expect("Failed to parse number");
    // Length of the line that our number is on
    let layer = input.sqrt().ceil();
    // Distance from corner of our line to middle of line (above center of spiral)
    let corner_to_mid = (layer - 1f64) / 2f64;
    // Distance to center of spiral
    let distance_to_center = layer.powi(2) - input;
    // Calculate total distance
    let total_distance = (corner_to_mid * 2f64) - distance_to_center;
    // Total distance as absolute value, also accounts for the case of input being in corners
    (((total_distance.abs()) - corner_to_mid.abs()) + corner_to_mid).abs() as u32
}
