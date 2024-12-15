use regex::Regex;

const INPUT: &str = include_str!("../../input/input_03.txt");

pub fn run() {
    let regex = Regex::new(r"(mul\(\d+,\d+\)|do(n't)?\(\))").unwrap(); // good regex ? this isnt real 
    let (mut a, mut b) = (0, 0);
    let mut dont = false; // shoutout dont gc
    for x in regex.find_iter(INPUT) {
        match x.as_str() {
            "do()" => dont = false,
            "don't()" => dont = true,
            x => {
                let (f, g) = x[4..x.len()-1].split_once(',').unwrap();
                let i = f.parse::<i32>().unwrap() * g.parse::<i32>().unwrap();
                a += i;
                if dont == false { b += i }; 
            }
        }
    }
    

    println!("part 1 answer {a}");
    println!("part 2 answer {b}");
}