use std::{path::Path, fs};
use itertools::Itertools;
use std::iter::zip;

fn lines_from(filename: impl AsRef<Path>, delimiter: &str) -> Vec<String> {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    
    raw_string.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

fn parse(filename: impl AsRef<Path>) -> Vec<(u64, u64)> {
    let (l, r) : (Vec<u64>, Vec<u64>) = lines_from(filename, "\n").iter()
        .map(|line| -> Vec<u64> {
            let t : String = line.split(':').skip(1).collect();
            t.trim().split_ascii_whitespace()
                .map(|x| x.parse::<u64>().unwrap()).collect()
        }).collect_tuple::<(_, _)>().unwrap();
    zip(l, r).collect()      
}

fn compute_distance(t: u64, tt: u64) -> u64 {
    /* NOTE: since the distance admits a closed-form expression,
        so does the inequality compute_distance(t, tt) < d for t. */
    (tt - t) * t
}

fn main() {
    let races = parse("./inputs/input6.txt");
    {
        let s1 : usize = races.iter().map(|(tt, d)| -> usize {
            (0..*tt).into_iter()
                .map(|t| compute_distance(t, *tt))
                .filter(|s| *s > *d).collect_vec().len()
        }).product();
        println!("{}", s1);
    }
    {
        let (tt, d) = races.iter().rev()
            .fold((0 , 0), |(tr, dr), (tl, dl)| {
                let etr : u32 = (tr as f64).log10().ceil() as u32;
                let edr : u32 = (dr as f64).log10().ceil() as u32;
                (10_u64.pow(etr) * tl + tr, 10_u64.pow(edr) * dl + dr)
            });
            let mut s2 = 0;
            for t in 0..tt {
                if compute_distance(t, tt) > d {
                    s2 += 1;
                }
            }
            println!("{}", s2);
        }

}