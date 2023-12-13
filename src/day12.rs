use std::{path::Path, fs, usize, vec};

use itertools::Itertools;

fn parse(filename: impl AsRef<Path>) -> Vec<(String, Vec<usize>)> {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    raw_string.split('\n').map(|l| {
        let ws = l.chars().position(|c| c == ' ').unwrap();
        (l[0..ws].replace('.', " ").to_string(), 
            l[ws+1..].split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect())
    }).collect()
}

// quite unoptimized algorithm. I will revisit for a faster 2nd star
fn arrangements(
    j0: usize, // starting index
    n: usize, // chunk size
    cgs: &[usize], // lengths of contiguous #
    fixed: &[usize], // where the '#'s are (in order)
) -> usize
{  
    let mut rett : usize = 0;
    if cgs.len() > 1 {
        let m = cgs[1..].iter().sum::<usize>() + cgs[1..].len() - 1;
        for j in 0..n - m - cgs[0] {
            let a = j + 1 + cgs[0];
            let q: usize = fixed.iter().position(|x| *x >= j0 + j + cgs[0]).unwrap_or(fixed.len());
            if !fixed.iter().all(|x| 
                (j0 + j <= *x && *x < j0 + j + cgs[0]) 
                    || *x >= j0 + j + cgs[0]) && fixed.len() != 0
                { continue; }
            rett += arrangements(
                j0 + a, n - (a), &cgs[1..], &fixed[q..]);
        }
    } else if cgs.len() == 1 {
        for j in 0..n-cgs[0]+1 {
            if fixed.iter().any(|x| {
                j0 + j > *x || *x >= j0 + j + cgs[0]
            }) { continue; }
            rett += 1;
        }
    }

    rett
}

fn overall(rec : &[&str], cgs: &[usize]) -> usize {
    if cgs.len() == 0 { 
        let u : usize = rec[0..].iter().map(|cg| 
            cg.chars().filter(|c| *c == '#')
                .count()
            ).sum();
        if u == 0 { return 1; } else { return 0; }
    }

    let fixed = rec[0].chars().enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(i, _)| i).collect_vec();
    if rec.len() == 1 {
        let m = cgs.iter().sum::<usize>() + cgs.len() - 1;
        return if m <= rec[0].len() { 
            arrangements(0, rec[0].len(), &cgs, &fixed)
        } else { return 0; }
    }

    let mut acc = 0;
    let j : usize = cgs.iter().position(|x| {
        acc += *x + 1;
        acc > 1 + rec[0].len() 
    }).unwrap_or(cgs.len());

    if j == 0 { 
        if fixed.len() == 0 { return overall(&rec[1..], &cgs); 
        } else { return 0;}
    } 

    let mut acc2 = 0;
    if fixed.len() == 0 {
        acc2 += overall(&rec[1..], &cgs[0..]);
    } 
    for i in 0..j {
        acc2 += arrangements(0, rec[0].len(), &cgs[0..i+1], &fixed)
            * overall(&rec[1..], &cgs[i+1..]);
    }

    acc2
}

fn main() {
    let records = parse("./inputs/input12.txt");
    {
        let rs : Vec<_> = records.iter().map(|(s, cgs)| {
            (s.split_whitespace().collect::<Vec<_>>(), cgs)
        }).collect();

        let s1 :usize = rs.iter().map(|(st, cgs)| {
            overall(&st[0..], &cgs[0..])
        }).sum();

        println!("{}", s1); 
    }
    {
        let records = records.iter().map(|e| {
            let mut s = e.0.clone();
            s.push('?'); s = s.repeat(5); s.pop();
            (s, e.1.repeat(5))
        }).collect_vec();
        
        let rs : Vec<_> = records.iter().map(|(s, cgs)| {
            (s.split_whitespace().collect::<Vec<_>>(), cgs)
        }).collect();

        let mut idx = 0; 
        let s2 :usize = rs.iter().map(|(st, cgs)| {
            println!("{}", idx);
            idx += 1;
            overall(&st[0..], &cgs[0..])
        }).sum();

        println!("{}", s2); 
    }
}