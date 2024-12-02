use std::collections::HashMap;

use utils::read_input;

fn main() {
    // input
    let raw_lines = read_input("inputs/d1.txt").expect("File should exist");

    let mut a: Vec<i32> = Vec::with_capacity(raw_lines.len());
    let mut b: Vec<i32> = Vec::with_capacity(raw_lines.len());
    for line in raw_lines {
        let mut iter = line.split_whitespace();
        a.push(iter.next().unwrap().parse().unwrap());
        b.push(iter.next().unwrap().parse().unwrap());
    }

    // part A
    a.sort();
    b.sort();

    let sum = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + (a - b).abs());

    println!("Part A: {}", sum);

    // part B
    let mut b_count = HashMap::<i32, i32>::new();
    for value in b.iter() {
        b_count
            .entry(*value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let sum = a.iter().fold(0, |acc, a| {
        let times = b_count.get(a).unwrap_or(&0);
        acc + a * times
    });
    println!("Part A: {}", sum);
}
