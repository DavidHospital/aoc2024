use std::{collections::HashSet, ops::Add, slice::Iter};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use utils::read_input;

const WORDS: [&'static str; 2] = ["XMAS", "SAMX"];
const LEN: usize = 4;

#[derive(EnumIter)]
enum Dir {
    LeftDown,
    Down,
    RightDown,
    Right,
}

impl From<&Dir> for Point {
    fn from(dir: &Dir) -> Self {
        match dir {
            Dir::LeftDown => Point(-1, 1),
            Dir::Down => Point(0, 1),
            Dir::RightDown => Point(1, 1),
            Dir::Right => Point(1, 0),
        }
    }
}

#[derive(Clone, Copy)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn mul(self, k: i32) -> Self {
        Self(self.0 * k, self.1 * k)
    }
}

struct Input(Vec<Vec<char>>);

impl Input {
    fn try_get(&self, point: Point) -> Option<char> {
        if point.0 < 0 || point.1 < 0 {
            return None;
        }
        self.0.get(point.0 as usize)?.get(point.1 as usize).cloned()
    }

    fn get(&self, point: Point) -> char {
        self.try_get(point).unwrap_or('.')
    }

    fn width(&self) -> usize {
        self.0.get(0).unwrap_or(&vec![]).len()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn iter_points(&self) -> impl Iterator<Item = Point> {
        (0..self.width())
            .cartesian_product(0..self.height())
            .map(|(i, j)| Point(i as i32, j as i32))
    }
}

fn main() {
    let raw_input = read_input("inputs/d4.txt").expect("file should exist");

    let words = HashSet::<&str>::from_iter(WORDS.into_iter());

    let input = Input(
        raw_input
            .into_iter()
            .map(|line| line.chars().collect())
            .collect(),
    );

    let width = input.width();
    let height = input.height();

    let mut sum = 0;
    for ((i, j), dir) in (0..width)
        .cartesian_product(0..height)
        .cartesian_product(Dir::iter())
    {
        let word: String = (0..LEN)
            .map(|idx| {
                let point = Point::from(&dir).mul(idx as i32) + Point(i as i32, j as i32);
                input.get(point)
            })
            .collect();

        if words.contains(word.as_str()) {
            sum += 1;
        }
    }

    println!("Part A: {}", sum);

    // Part B
    let mut sum = 0;

    for point in input.iter_points() {
        if input.get(point) != 'A' {
            continue;
        }

        // first diagonal
        let first = input.get(point + Point(-1, -1));
        if first != 'M' && first != 'S' {
            continue;
        }

        let second = input.get(point + Point(1, 1));
        if second != 'M' && second != 'S' {
            continue;
        }

        if second == first {
            continue;
        }

        // second diagonal
        let first = input.get(point + Point(1, -1));
        if first != 'M' && first != 'S' {
            continue;
        }

        let second = input.get(point + Point(-1, 1));
        if second != 'M' && second != 'S' {
            continue;
        }

        if second == first {
            continue;
        }

        sum += 1;
    }

    println!("Part B: {}", sum);
}
