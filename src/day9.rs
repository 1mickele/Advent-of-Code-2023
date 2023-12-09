use std::{path::Path, fs};
use itertools::Itertools;

fn parse(filename: impl AsRef<Path>) -> Vec<Vec<i64>>
{
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");

    raw_string.split('\n')
        .map(|l| l.split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect()
        )
        .collect()
}

fn compute_history(data: Vec<i64>) -> (i64, i64) {
    let mut differences: Vec<_> = Vec::from([data]);
    loop {
        let last : &Vec<i64> = differences.last().unwrap();
        if last.iter().all(|x| *x == 0) 
            { break; }

        differences.push(
            last.iter().tuple_windows::<(_,_)>()
                .map(|(a, b)| *b - *a).collect()
        );
    }

    let (mut l, mut r) = (0_i64, 0_i64);
    for i in (1..differences.len()).rev() {
        r = differences[i - 1].last().unwrap() + r;
        l = differences[i - 1].first().unwrap() - l;
    }

    (r, l)
}

fn main() {
    let data = parse("./inputs/input9.txt");
    
    let (s1, s2) : (i64, i64) = data.into_iter()
        .map(|d| compute_history(d))
        .fold((0, 0), |(a, b), (c, d)| (a + c, b + d));
    println!("{}", s1);
    println!("{}", s2);
    
}