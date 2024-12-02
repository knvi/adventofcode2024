use std::{cmp::{max, min}, collections::HashMap};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/input_02.txt");

pub fn run() {
    let input: usize = INPUT
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>()
        .iter()
        .filter(|x| {
            x
                .iter()
                .tuple_windows()
                .map(|(a, b)| a - b)
                .all(|y| (1..=3).contains(&y.abs()) && y.signum() == (x[0] - x[1]).signum())
        })
        .count()
        .into();

    println!("part 1 answer {input}");
}