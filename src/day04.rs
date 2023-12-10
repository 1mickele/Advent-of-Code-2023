use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools; // 0.9.0
use std::collections::HashSet;

fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
    let tab : Vec<&str> = input.split(": ").collect();
    let card : (Vec<i32>, Vec<i32>) = tab[1].split(" | ")
        .map(|p| {
            p.split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        }).next_tuple().unwrap();

    card
}

fn main() -> io::Result<()>  {
    {
        let file = File::open("./inputs/input4.txt")?;
        let reader = BufReader::new(&file);
        let s1 = reader
            .lines()
            .map_ok(parse)
            .map_ok(|(v, w)| -> i32 {
                let a: HashSet<i32> = HashSet::from_iter(v.iter().cloned());
                let b: HashSet<i32> = HashSet::from_iter(w.iter().cloned());
                let c: HashSet<i32> = a.into_iter().filter(|e| b.contains(e)).collect();
                // println!("{}", c.len());

                return if c.len() > 0 { 2_i32.pow((c.len() - 1) as u32) } else { 0 };
            }).fold_ok(0, |a, b| a + b);
        
        println!("{}", s1.unwrap());
    }
    {
        let file = File::open("./inputs/input4.txt")?;
        let reader = BufReader::new(&file);
        let cards : Vec<Vec<usize>> = reader
            .lines()
            .map(|x| x.unwrap())
            .map(parse)
            .enumerate()
            .map(|(i, (v, w))| {
                let a: HashSet<i32> = HashSet::from_iter(v.iter().cloned());
                let b: HashSet<i32> = HashSet::from_iter(w.iter().cloned());
                let c: HashSet<i32> = a.into_iter().filter(|e| b.contains(e)).collect();

                Vec::from_iter(i+1..i+c.len()+1)        
            }).collect();
            
        let mut s2 = cards.len();
        let mut card_accumulator : Vec<usize> = vec![1; cards.len()];

        for i in 0..card_accumulator.len() {
            let c = card_accumulator[i];
            if c > 0 {
                for j in &cards[i] {
                    card_accumulator[*j] += c;
                    s2 += c;
                }
            }
        }
   
        println!("{}", s2);
    }
        
    Ok(())
}
