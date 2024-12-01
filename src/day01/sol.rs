use std::{cmp::{max, min}, collections::HashMap};

const INPUT: &str = include_str!("../../input/input_01.txt");

pub fn run() {
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();

    for line in INPUT.lines() {
        let pair = line.split_once("   ").unwrap(); // dumb 3 spaces
        // println!("{} / {} is the pair", pair.0, pair.1);
        left.push(pair.0.parse::<i64>().unwrap());
        right.push(pair.1.parse::<i64>().unwrap());
    }

    left.sort();
    right.sort();

    let mut dist = 0;

    for i in 0..left.len() {
        dist += max(left[i], right[i]) - min(left[i], right[i])
    }

    println!("part 1 result: {dist}");

    // ---- part 2
    
    // lets just do a hashmap for this ig

    let mut similarity_map = HashMap::<i64, i64>::new();
    let mut similarity_score: i64 = 0;

    for i in right {
        *similarity_map.entry(i).or_insert(0) += 1;
    }

    for i in left {
        let score = similarity_map.get(&i).unwrap_or(&0);

        similarity_score += score*i;
    }

    println!("part 2 results: {similarity_score}");


}