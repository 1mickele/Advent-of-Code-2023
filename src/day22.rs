use std::fs;
use itertools::{Itertools, iproduct};

type Point = (usize, usize, usize);

fn parse() -> Vec<(Point, Point)> {
    let raw_string = fs::read_to_string("./inputs/input22.txt")
        .expect("Unable to read file");

    let mut sn = raw_string.split('\n').map(|l| {
        l.split('~').map(|b| 
            b.split(',').map(|x| x.parse::<usize>().unwrap() )
                .collect_tuple::<Point>().unwrap()
        ).collect_tuple::<(_,_)>().unwrap()
    }).collect_vec();
    sn.sort_by_key(|(_, q)| q.2); // it always holds true _.2 <= q.2 
    return sn
}

const SIDE : usize = 10;

fn collapse(snapshot : &mut Vec<(Point, Point)>) -> usize {
    let mut collapsed = 0_usize;
    let mut floors: Vec<[bool; SIDE * SIDE]> = vec![[false; SIDE * SIDE]];
    for (p, q) in snapshot.iter_mut() {
        
        let z = (0..p.2).rev().find_map(|z| {
            floors.get(z).map(|floor| {
                let c = iproduct!((p.0..(q.0 + 1)), p.1..(q.1 + 1)).any(|(i, j)| {
                    floor[i * SIDE + j]
                });
                if c { Some(z + 1) } else { None }
            }).flatten()
        }).unwrap_or(0);

        if z != p.2 { collapsed += 1; }
        q.2 = (q.2 - p.2) + z; p.2 = z;
        
        for (i, j, k) in 
            iproduct!((p.0..(q.0 + 1)), p.1..(q.1 + 1), z..(z + q.2 - p.2 + 1))
        {
            if floors.get(k).is_none() 
                { floors.push([false; SIDE * SIDE]); }
            *floors.get_mut(k).unwrap().get_mut(i * SIDE + j).unwrap() = true;
        }
    }

    // dbg!(&floors, &snapshot, collapsed);

    collapsed
}

fn main() {
    let mut snapshot = parse();
    collapse(&mut snapshot);
    {
        let s1 = (0..snapshot.len()).map(|i| {
            let mut nconfiguration = Vec::new(); 
            nconfiguration.extend_from_slice(&snapshot[..i]);
            nconfiguration.extend_from_slice(&snapshot[i+1..]);

            if collapse(&mut nconfiguration) == 0 { true } else { false }
        }).filter(|x| *x).count();
        println!("{}", s1);
    }
    {
        let s2 : usize = (0..snapshot.len()).map(|i| {
            let mut nconfiguration = Vec::new(); 
            nconfiguration.extend_from_slice(&snapshot[..i]);
            nconfiguration.extend_from_slice(&snapshot[i+1..]);

            collapse(&mut nconfiguration)
        }).sum();
        println!("{}", s2);
    }
}
