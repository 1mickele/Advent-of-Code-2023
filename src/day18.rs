use std::{fs, usize, isize};

use itertools::Itertools;

fn parse() -> (Vec<(char, isize)>, Vec<(char, isize)>) {
    let raw_string = fs::read_to_string("./inputs/input18.txt").expect("Unable to read file");
    raw_string.split('\n').map(|l| -> ((char, isize), (char, isize)) {
        let ls = l.split(' ').collect_vec();
        let b1 = ls[0].chars().next().unwrap();
        let c1 = ls[1].parse::<isize>().unwrap();

        let b2 = match ls[2].as_bytes().get(ls[2].len()-2) {
            Some(b'0') => 'R',
            Some(b'1') => 'D',
            Some(b'2') => 'L',
            Some(b'3') => 'U',
            _ => 'x'
        };
        
        let c2 = isize::from_str_radix(&ls[2][2..ls[2].len() - 2], 16)
            .expect("unable to parse hex");

        ((b1, c1), (b2, c2))
    }).into_iter().unzip() // into_iter().unzip().collect()
}

fn get_area(input : &Vec<(char, isize)>) {
    fn diff(
        (u, v) : (&mut isize, &mut isize), 
        (p, q) : (isize, isize)
    ) -> (isize, isize) {
        let (a, b) = (p - *u, q - *v);
        (*u, *v) = (p, q);
        (a, b)
    }

    // hard-coded values (input tailored)
    let (mut u, mut v) = (0_isize, 0_isize);
    let mut vertices = input.iter().tuple_windows::<(_, _)>()
        .scan((0_isize, 0_isize),
        |(i, j), ((c1, f1), (c2, _))| {
            match (*c1, *c2) {
                ('R', 'D') => {
                    let (_, b) = diff((&mut u, &mut v), (0, 1));
                    *j += *f1 + b; Some((*i, *j)) 
                }
                ('R', 'U') => {
                    let (_, b) = diff((&mut u, &mut v), (0, 0));
                    *j += *f1 + b; Some((*i, *j))                  
                }
                ('R', _) => { *j += *f1; Some((*i, *j)) }
                ('L', 'U') => {
                    let (_, b) = diff((&mut u, &mut v), (1, 0));
                    *j += -*f1 + b; Some((*i, *j)) 
                }
                ('L', 'D') => {
                    let (_, b) = diff((&mut u, &mut v), (1, 1));
                    *j += -*f1 + b; Some((*i, *j))                  
                }
                ('L', _) => { *j -= *f1; Some((*i, *j)) }
                ('D', 'R') => {
                    let (a, _) = diff((&mut u, &mut v), (0, 1));
                    *i += *f1 + a; Some((*i, *j)) 
                }
                ('D', 'L') => {
                    let (a, _) = diff((&mut u, &mut v), (1, 1));
                    *i += *f1 + a; Some((*i, *j))                
                }
                ('D', _) => { *i += *f1; Some((*i, *j)) }
                ('U', 'R') => {
                    let (a, _) = diff((&mut u, &mut v), (0, 0));
                    *i += -*f1 + a; Some((*i, *j)) 
                }
                ('U', 'L') => {
                    let (a, _) = diff((&mut u, &mut v), (1, 0));
                    *i += -*f1 + a; Some((*i, *j))                  
                }
                ('U', _) => { *i -= *f1; Some((*i, *j)) }                  
                (_, _) => { Some((*i, *j)) }
            }
        }).collect_vec();
    vertices.push((0, 0));

    let (yn, xn) = vertices.last().unwrap();
    let (y0, x0) = vertices.last().unwrap();

    let t = (vertices.iter().tuple_windows().map(|((y1, x1), (y2, x2))| 
        (y2 + y1) * (x1 - x2)
    ).sum::<isize>() + (yn + y0) * (xn - x0)) / 2;
    println!("{}", t);
}

// result: 129895
fn _estimate_size(input : &Vec<(char, isize)>) -> (isize, isize, isize, isize) {
    let it = input.iter().scan((0_isize, 0_isize), |(i, j), (c, f)| {
        
        match *c {
            'R' => { *j += *f; Some((*i, *j)) },
            'L' => { *j -= *f; Some((*i, *j)) },
            'U' => { *i -= *f; Some((*i, *j)) },
            'D' => { *i += *f; Some((*i, *j)) },
            _ => None
        }
    });
    let (mut u,mut l, mut d, mut r) 
        = (isize::MAX, isize::MAX, isize::MIN, isize::MIN);
    for (y, x) in it {
        if x > r { r = x; }
        else if x < l { l = x; }

        if y > d { d = y; }
        else if y < u { u = y }
    }

    (2+r-l, 2+d-u, 1-l, 1-u)
}

fn _dig(
    (w, h, x0, y0): (isize, isize, isize, isize), 
    input: &Vec<(char, isize)>) 
{
    let mut area : Vec<u8> = vec![b'.'; (w * h) as usize];
    let (mut x, mut y) = (x0, y0);
    for (c, f) in input {
        match *c {
            'R' => { 
                for k in 0..*f {
                    area[(y * w + x + k) as usize] = b'#';
                }
                x += f;
            }
            'L' => { 
                for k in 0..*f {
                    area[(y * w + x - k) as usize] = b'#';
                }
                x -= f;
            }
            'U' => { 
                for k in 0..*f {
                    // dbg!(x, y, f);
                    area[((y - k) * w + x) as usize] = b'#';
                }
                y -= f;
            }
            'D' => { 
                for k in 0..*f {
                    area[((y + k) * w + x) as usize] = b'#';
                }
                y += f;
            }
            _ => {  }
        }
    }

    // fill
    
    let mut todig : Vec<(isize, isize)> = vec![(y0+1, x0+1)]; // hard-coded value
    while let Some((u, v)) = todig.pop() {
        let nghb = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1)
        ];
        for (y, x) in nghb {
            if let Some(c) = area.get_mut(((y + u) * w + (v + x)) as usize) {
                if *c != b'#' {
                    *c = b'#';
                    todig.push((y + u, x + v));
                }
            }
        }


    }

    let s1 = area.iter().filter(|c| **c == b'#').count();
    println!("{}", s1);
}


fn main() {
    let (input1, input2) = parse();
    get_area(&input1);
    get_area(&input2);
}