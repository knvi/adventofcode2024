use std::{cmp::{max, min}, collections::HashMap};

const INPUT: &str = include_str!("../../input/input_01.txt");

pub fn run() {
    
    let (mut left, mut right): (Vec<i64>, Vec<i64>) = INPUT
        .lines()
        .map(|line| {
            let pair = line.split_once("   ").unwrap();
            (pair.0.parse::<i64>().unwrap(), pair.1.parse::<i64>().unwrap())
        })
        .unzip();

    left.sort();
    right.sort();

    let dist: i64 = left
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, l)| max(l, right[i]) - min(l, right[i]))
        .sum();

    println!("part 1 result: {dist}");

    // ---- part 2
    
    // lets just do a hashmap for this ig

    let similarity_map: HashMap<_, _> = right
        .iter()
        .fold(HashMap::new(), |mut map, &item| {
            *map.entry(item).or_insert(0) += 1;
            map
        });

    let similarity_score: i64 = left
        .iter()
        .map(|&item| item * similarity_map.get(&item).copied().unwrap_or(0))
        .sum();
        

    println!("part 2 results: {similarity_score}");


}