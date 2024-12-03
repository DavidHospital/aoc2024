use regex::Regex;
use utils::read_input_string;

fn main() {
    let raw_input = read_input_string("inputs/d3.txt").expect("File should exist");
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let sum = re
        .captures_iter(&raw_input)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .fold(0, |acc, (a, b)| acc + a * b);

    println!("Part A: {}", sum);

    // Part B
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mul_re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut r#do = true;
    let mut sum = 0;
    for m in re.find_iter(&raw_input) {
        match m.as_str() {
            m if m == "do()" => r#do = true,
            m if m == "don't()" => r#do = false,
            _ => {
                if r#do {
                    sum += mul_re
                        .captures_iter(&m.as_str())
                        .map(|caps| {
                            let (_, [a, b]) = caps.extract();
                            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
                        })
                        .fold(0, |acc, (a, b)| acc + a * b);
                }
            }
        }
    }
    println!("Part B: {}", sum);
}
