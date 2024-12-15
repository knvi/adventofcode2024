use std::{cmp::{max, min}, collections::HashMap};
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/input_02.txt");

fn is_safe(levels: &[i32]) -> bool {
    // all decreasinng or all increasing
    let mut is_safe = false;
    is_safe |= levels.iter().tuple_windows().all(|(a, b)| a < b); 
    is_safe |= levels.iter().tuple_windows().all(|(a, b)| a > b); 
    is_safe && levels.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(a - b).abs())) // spent 15 minutes figuring out that i didnt put the =3
}

fn is_safe_b(levels: &[i32]) -> bool {
    (0..levels.len()).any(|i| {
        let mut y = levels.to_vec();
        y.remove(i);
        is_safe(&y)
    })
}

pub fn run() {
    let (mut a, mut b) = (0, 0);
    for l in INPUT.lines() {
        let x = l.split_whitespace().map(|w| w.parse::<i32>().unwrap()).collect::<Vec<_>>();
        if (is_safe(&x)) { a += 1; }
        if (is_safe_b(&x)) { b += 1; }
    }
        
    // let a = input
    //     .iter()
    //     .filter(|x| {
    //         x
    //             .iter()
    //             .tuple_windows()
    //             .map(|(a, b)| a - b)
    //             .all(|y| (1..=3).contains(&y.abs()) && y.signum() == (x[0] - x[1]).signum())
    //     })
    //     .count()
    //     .into();
        
    println!("part 1 answer {a}");
    println!("part 2 answer {b}");
}