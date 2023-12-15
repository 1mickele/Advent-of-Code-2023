use std::fs;

use itertools::Itertools;

fn hash(data : &str) -> usize {
    data.as_bytes().iter().fold(0_usize, |acc, c| {
        ((acc + (*c as usize)) * 17) % 256
    })
}

fn main() {
    let raw_string = fs::read_to_string("./inputs/input15.txt")
        .expect("Unable to read file").split('\n').take(1).collect::<String>();
    {
        println!("{}", raw_string.split(',').map(hash).sum::<usize>());
    }
    {
        const DEF : Vec<(String, usize)> = Vec::new();
        let mut boxes : [Vec<(String, usize)>; 256] = [DEF; 256];

        let instructions = raw_string.split(',').map(|l| {
                let i = l.chars().position(|c| c == '=' || c == '-').unwrap();
                (l[0..i].to_string(), l.as_bytes()[i], l[i+1..].parse::<usize>().ok())
            }).collect_vec();

        for (label, c, flen) in instructions {
            let i = hash(&label);
            match (c, flen) {
                (b'=', Some(n)) => {
                    let x = boxes[i]
                        .iter_mut().find(|(l, _)| { label == *l });
                    match x {
                        Some((_, on)) => { *on = n; }
                        None => { boxes[i].push((label, n)); }
                    }
                },
                (b'-', None) => {
                    let q = boxes[i].iter().position(|(l,_)| *l == label);
                    if q.is_some() 
                        { boxes[i].remove(q.unwrap()); }
                }
                _ => {
                    println!("!");
                }
            }
        } 

        let s2 : usize = boxes.iter().enumerate().map(|(i, v)|
            v.iter().enumerate().fold(0_usize, |acc, (j, (_, u))| {
                acc + (i + 1) * (j + 1) * u
            })
        ).sum();

        println!("{}", s2);
    }
}