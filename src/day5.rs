use std::{path::Path, fs};
use itertools::Itertools;

fn lines_from(filename: impl AsRef<Path>, delimiter: &str) -> Vec<String> {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    
    raw_string.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

fn parse(data: &Vec<String>) ->
(
        Vec<u64>, // seeds
        Vec<Vec<(u64, u64, u64)>> // maps
)
{
    let seeds_string = &data[0];
    let seeds : Vec<u64> = seeds_string[seeds_string.find(": ").unwrap() + 2..]
        .split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

    let maps : Vec<Vec<(u64, u64, u64)>> = data[1..].iter()
        .map(|block| {
            let mut m = block.split("\n").skip(1).map(|line| -> (u64, u64, u64) {
                line.split(' ').map(|x| x.parse::<u64>().unwrap())
                    .collect_tuple::<(_, _, _)>().unwrap() // ..a triple
            }).collect::<Vec<_>>();
            m.sort_by_key(|(_,u,_)| *u); //.. a map-block
            m
        }).collect();

    (seeds, maps)
}

fn compute_mapping(x: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (v, u, l) in map {
        if *u <= x && x < u + *l {
            return v + x - u;
        }
    }
    return x;
}

fn compute_mapping_intervals(
    (x, t): (u64, u64), 
    map: &Vec<(u64, u64, u64)>
) -> Vec<(u64, u64)> 
{
    let mut res: Vec<(u64, u64)> = Vec::new();
    let (mut a, mut i) = (x, 0);
    while i < map.len() && a != x + t {
        let (v,u,l) = map[i];
        if u <= a && a < u + l {
            let b = (x + t).min(u + l);
            res.push((v + a - u, b - a));
            (a, i) = (b, i+1);
        } else if u <= a {
            i+=1;
        } else { // u > a
            let b = (x + t).min(u);
            res.push((a, b - a));
            a = b;
        }
    }
    if i == map.len() && a != x + t { 
        res.push((a,x + t - a)); 
    }

    res
}

fn main() {
    let lines = lines_from("./inputs/input5.txt", "\n\n");
    let (seeds, maps) = parse(&lines);
    {
        let s1 = seeds.iter().map(|x| 
            maps.iter().fold(*x, |t, m| 
                compute_mapping(t, m))
        ).min().unwrap();
        println!("{}", s1);
    }
    {
        let mut seeds_interval : Vec<(u64, u64)> = seeds.chunks(2)
            .map(|x| (x[0], x[1]))
            .collect();
        for map in maps {
            let mut acc : Vec<(u64, u64)> = Vec::new();
            for (x, t) in &seeds_interval {
                acc.extend(compute_mapping_intervals((*x, *t), &map));
            }

            seeds_interval = acc;
        }

        let s2 = seeds_interval.iter().map(|x| x.0).min().unwrap();
        println!("{}", s2);
    }
}
