use std::{path::Path, fs};
use std::collections::HashSet;

use itertools::enumerate;

fn parse(filename: impl AsRef<Path>) -> (Vec<(usize, usize)>, usize) {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    let width = raw_string.chars()
        .position(|c| c == '\n').unwrap() + 1;
    (Vec::from_iter(raw_string.chars().enumerate()
        .filter(|(i, x)| *x == '#')
        .map(|(i, _)| (i / width, i % width))), width - 1)
}

fn expand(skymap: &Vec<(usize, usize)>, width: usize, amount: usize) -> Vec<(usize, usize)> {
    let mut eskymap = skymap.clone();
    let empty_cols = (0..width).into_iter()
        .filter(|j| (0..width).into_iter()
            .all(|i| !skymap.contains(&(i,*j))));
    for cl in empty_cols {
        for i in 0..skymap.len() {
            if skymap[i].1 > cl { eskymap[i].1 += amount; }
        }
    }
    let empty_rows = (0..width).into_iter()
    .filter(|i| (0..width).into_iter()
        .all(|j| !skymap.contains(&(*i,j))));

    for cl in empty_rows {
        for i in 0..skymap.len() {
            if skymap[i].0 > cl { eskymap[i].0 += amount; }
        }
    }

    eskymap
}

fn main() {
    let (sky, width) = parse("./inputs/input11.txt");
    {
        let esky = expand(&sky, width, 1);
        let mut s1 = 0;
        for i in 0..esky.len() {
            for j in i..esky.len()  {
                s1 += (*(&esky[i].1) as isize).abs_diff(*&esky[j].1 as isize) +
                    (*&esky[i].0 as isize).abs_diff(*&esky[j].0 as isize);
            }
        }
        println!("{}", s1);
    }
    {
        let esky = expand(&sky, width, 1000000 - 1);
        let mut s1 = 0;
        for i in 0..esky.len() {
            for j in i..esky.len()  {
                s1 += (*(&esky[i].1) as isize).abs_diff(*&esky[j].1 as isize) +
                    (*&esky[i].0 as isize).abs_diff(*&esky[j].0 as isize);
            }
        }
        println!("{}", s1);
    }
}