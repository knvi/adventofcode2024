use std::{cmp::{max, min}, collections::HashMap};
use itertools::Itertools;
use regex::Regex;

const INPUT: &str = include_str!("../../input/input_03.txt");

pub fn run() {
    let mut regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut a = 0;
    for (_, [num1, num2]) in regex.captures_iter(INPUT).map(|x| x.extract()) {
        a += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap()
    }

    println!("part 1 answer {a}");
    // println!("part 2 answer {b}");
}