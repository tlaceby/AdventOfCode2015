// --- Day 1: Not Quite Lisp ---
// https://adventofcode.com/2015/day/1
// Key rules:
// - Open Paren -> incriment floor
// - Close Paren -> decriment flor

pub fn not_quiet_lisp() {
    println!("\nDay 01 - Advent Of Code 2015:");
    let mut floor = 0;
    let mut set_floor_index = false;
    let contents = std::fs::read_to_string("./inputs/day1").unwrap();

    for (indx, char) in contents.chars().enumerate() {
        floor += if char == '(' { 1 } else { -1 };

        // Check for the first negative floor and then report that.
        if floor < 0 && !set_floor_index {
            println!(" - Basement Floor Index: {}", indx + 1);
            set_floor_index = true;
        }
    }

    println!(" - Final floor: {}", floor);
}
