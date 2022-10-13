// --- Day 2: I Was Told There Would Be No Math ---
// https://adventofcode.com/2015/day/2

use std::fs::read_to_string;

pub fn i_was_told_no_math() {
    let mut total_ribbon: i64 = 0;
    let mut total_wrapping: i64 = 0;
    let contents = read_to_string("./inputs/day2").unwrap();

    // Parse line for l_w_h and add total to paper count.
    for (_, line) in contents.lines().enumerate() {
        let mut nums_slice = line.split("x");

        // // Parse out the individual numbers inside the string.
        let length = nums_slice.next().unwrap().parse::<i64>().unwrap();
        let width = nums_slice.next().unwrap().parse::<i64>().unwrap();
        let height = nums_slice.next().unwrap().parse::<i64>().unwrap();

        total_ribbon += calculate_ribbon_lengths(length, width, height);
        total_wrapping += total_wrapping_paper(length, width, height);
    }

    println!(
        "Wrapping Paper: {} ft. Ribbon: {} ft.",
        total_wrapping, total_ribbon
    );
}

fn calculate_ribbon_lengths(len: i64, width: i64, height: i64) -> i64 {
    let mut sides = vec![len, width, height];
    sides.sort();

    let ribbon_length = (2 * sides[0]) + (2 * sides[1]);
    let bow_length = len * width * height;

    return ribbon_length + bow_length;
}

// The total wrapping paper is the surface area + smallest side.
fn total_wrapping_paper(length: i64, width: i64, height: i64) -> i64 {
    let a = length * width;
    let b = width * height;
    let c = height * length;

    let surface_area = (2 * a) + (2 * b) + (2 * c);
    let smallest: i64;

    // Determine smallest of a, b, c
    if a <= b && a <= c {
        smallest = a;
    } else if b <= a && b <= c {
        smallest = b;
    } else {
        smallest = c;
    }

    return smallest + surface_area;
}
