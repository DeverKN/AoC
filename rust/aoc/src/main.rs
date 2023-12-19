use std::fs;
mod day10;
mod day12;
mod day19;

fn main() {
    // let val = fs::read_to_string("/Users/deverkemmenash/Desktop/2023/AoC/rust/aoc/inputs/day_1.txt")
    //     .expect("Unable to read file")
    //     .lines()
    //     .map(|line| handle_line(translate(line.to_string())))
    //     .fold(0, |acc, x| acc + x);
    // println!("result is: {}", val);
    day19::main();
}

struct Pattern {
    pattern: String,
    replacement: String,
}

fn translate_helper(s: String, index: usize, patterns: [Pattern; 9]) -> String {
    if s == "" {
        return s;
    } else if index == patterns.len() {
        return s[0..1].to_string() + &translate(s[1..].to_string());
    } else {
        let pattern = &patterns[index];
        if s.starts_with(&pattern.pattern) {
            return "".to_string() + &pattern.replacement + &translate(s[pattern.pattern.len()..].to_string());
        } else {
            return translate_helper(s, index + 1, patterns);
        }
    }
}

fn translate(s: String) -> String {
    let patterns = [
        Pattern { pattern: "one".to_string(), replacement: "1".to_string() },
        Pattern { pattern: "two".to_string(), replacement: "2".to_string() },
        Pattern { pattern: "three".to_string(), replacement: "3".to_string() },
        Pattern { pattern: "four".to_string(), replacement: "4".to_string() },
        Pattern { pattern: "five".to_string(), replacement: "5".to_string() },
        Pattern { pattern: "six".to_string(), replacement: "6".to_string() },
        Pattern { pattern: "seven".to_string(), replacement: "7".to_string() },
        Pattern { pattern: "eight".to_string(), replacement: "8".to_string() },
        Pattern { pattern: "nine".to_string(), replacement: "9".to_string() },
    ];
    return translate_helper(s, 0, patterns)
}

fn handle_line(s: String) -> u32 {
    println!("translated: {}", s);
    let nums = s.chars()
                        .filter(|c| c.is_numeric())
                        // .map(|c| c.to_string())
                        .collect::<Vec<char>>();
                        // .map(|c| c.to_digit(10).expect("A non-numeric value made it past the filter"))
                        // .collect::<Vec<u32>>();
    let first = nums.first().expect("no first value");
    let last = nums.last().expect("no last value");
    let val = (first.to_string() + &last.to_string()).parse().expect("Not a number");
    println!("first: {}, last: {}, val: {}", first, last, val);
    return val
}