use std::fs;

use itertools::Itertools;

fn main() {
    let mut x = -1;
    let raw_string = fs::read_to_string("./inputs/input13.txt").expect("Unable to read file");
    let (s1,s2) : (isize, isize) = raw_string.split("\n\n")
        .map(|q| {
            x+= 1;
            let w = q.chars().position(|c| c == '\n').unwrap() as isize;
            let h = (q.len() as isize + 1) / (w + 1) ;
            let q = q.as_bytes();

            let refl = (0..w-1).map(|s| {
                    let d = (w-s-2).min(s) + 1;
                    (s + 1, h * d - (0..h).map(|i| {
                        (0..d).map(|j| {
                            q[(i * (w + 1) + s - j) as usize] 
                                == q[(i * (w + 1) + s + j + 1) as usize]
                        }).filter(|b| *b).count()
                    }).sum::<usize>() as isize)                
                })
                .filter(|(_, e)| *e == 0 || *e == 1)
                .chain(
                (0..h-1).map(|s| {
                    let d = (h-s-2).min(s) + 1;
                    (100 * (s + 1), d * w - (0..w).map(|j| {
                        (0..d).map(|i| {
                            q[((s - i) * (w + 1) + j) as usize] 
                                == q[((s + i + 1) * (w + 1) + j) as usize]
                        }).filter(|b| *b).count()
                    }).sum::<usize>() as isize)                
                })
                .filter(|(_, e)| *e == 0 || *e == 1)
                ).collect_vec();            

            if refl[0].1 == 0 { return (refl[0].0, refl[1].0); } 
            else { return (refl[1].0, refl[0].0); }

        }).fold((0,0), |(a,b), (c,d)| (a+c, b+d));
    println!("{}, {}", s1, s2);
}