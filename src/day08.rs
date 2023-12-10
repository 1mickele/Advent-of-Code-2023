use std::{path::Path, fs};
use std::collections::HashMap;
use num::integer::lcm;

use itertools::Itertools;

fn parse(filename: impl AsRef<Path>) -> (
    Vec<bool>,
    HashMap<String, (String, String)>
) {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");

    let directions = raw_string.split("\n\n").take(1)
        .collect::<String>().chars()
        .map(|c| if c == 'R' {true} else {false})
        .collect_vec();

    let paths : HashMap<String, (String, String)> = raw_string.split("\n\n").skip(1)
        .collect::<String>().split('\n')
        .map(|line| -> (String, (String, String)){
            (line[0..3].to_string(),
                (line[7..10].to_string(), line[12..15].to_string()))
        })
        .collect();

    (directions, paths)
}

fn main() {
    let (dir, path) = parse("./inputs/input8.txt");
    {
        let mut current = "AAA";
        let mut s1 = 0;
        for d in dir.iter().cycle() {
            if current == "ZZZ" { break; }
            current =
                if *d == false {&path[current].0} 
                else {&path[current].1};
            s1 += 1;
        }
        println!("{}", s1)
    }
    {
        let mut currents: Vec<&String> = path.iter()
            .map(|(s, _)| s)
            .filter(|s| { *s.as_bytes().last().unwrap() == b'A' })
            .collect_vec();    

        let recurrence_time = currents.iter_mut().map(|c| -> u64 {
            let mut t = 0;
            for d in dir.iter().cycle() {
                if c.ends_with('Z') { break; }
                *c =
                    if *d == false {&path[*c].0} 
                    else {&path[*c].1};
                t += 1;  
            }
            t
        }).collect_vec();

        let s2 = recurrence_time.into_iter()
            .reduce(|a, b| lcm(a, b)).unwrap();

        println!("{}", s2);    
    }
}