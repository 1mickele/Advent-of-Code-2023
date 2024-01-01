use std::fs::{self, File};
use std::collections::HashMap;
use std::io::Write;
use std::isize;
use itertools::{Itertools, iproduct};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileType {
    Plot,
    Rock,
    Marked
}


fn parse()
    -> (HashMap<(isize, isize), TileType>, isize)
{
    let raw_string = fs::read_to_string("./inputs/input21.txt")
        .expect("Unable to read file");

    let l = raw_string.chars().position(|c| c == '\n').unwrap() as isize;

    (HashMap::from_iter(raw_string.as_bytes().iter().filter(|&&c| c != b'\n')
        .enumerate().map(|(i, &c)| {
            ((i as isize / l, i as isize % l as isize), match c {
                b'.' => TileType::Plot,
                b'#' => TileType::Rock,
                b'S' => TileType::Marked,
                _ => TileType::Plot
            })
        })), l)
}

fn _printf(map : &HashMap<(isize, isize), TileType>, l: isize) {
    let mut file = File::create("output.txt").unwrap();
    file.write_all(&iproduct!((0..l), (0..l+1)).map(|(i, j)| {
        match map.get(&(i, j)) {
            Some(TileType::Marked) => b'O',
            Some(TileType::Rock) => b'#',
            Some(TileType::Plot) => b'.',
            None => b'\n'
        }
    }).collect_vec()[0..]).unwrap();
}

fn _replicate(source : &HashMap<(isize, isize), TileType>, l: isize, n: isize)
    -> (HashMap<(isize, isize), TileType>, isize)
{
    let mut target = HashMap::new();
    for i in 0..2*n+1 {
        for j in 0..2*n+1 {
            target.extend(source.iter().map(|((a, b), x)| {
                ((i * l + a, j * l + b), match x {
                    TileType::Rock => TileType::Rock,
                    _ => TileType::Plot
                })                
            }));
        }
    }
    *target.get_mut(&(n * l + l / 2, n * l + l / 2)).unwrap() = TileType::Marked;

    (target, (2*n + 1) * l)
}

fn step(map : &mut HashMap<(isize, isize), TileType>, l: isize) 
{
    let mut nmap = map.clone();
    for i in 0_isize..l {
        for j in 0..l {
            if map.get(&(i ,j)) == Some(&TileType::Marked) {
                *nmap.get_mut(&(i, j)).unwrap() = TileType::Plot;
                for (di, dj) in [(i+1, j), (i-1, j), (i, j+1), (i,j-1)] {
                    let p = map.get(&(di, dj));
                    if p == Some(&TileType::Plot) || p == Some(&TileType::Marked) {
                        *nmap.get_mut(&(di, dj)).unwrap() = TileType::Marked;
                    }
                }
            }
        }
    }
    *map = nmap;
}

fn setup(map : &HashMap<(isize, isize), TileType>, 
    l: isize, (i,j) : (isize, isize)
) -> HashMap<(isize, isize), TileType> {
    let mut mmap = map.clone();
    *mmap.get_mut(&(l / 2, l / 2)).unwrap() = TileType::Plot;
    *mmap.get_mut(&(i, j)).unwrap() = TileType::Marked;
    return mmap;
}

fn explore(
    mut source : HashMap<(isize, isize), TileType>, l: isize, s: isize
) -> usize {
    for _ in 1..s + 1 {
        step(&mut source, l);
    }
    source.values().filter(|&&t| t == TileType::Marked).count()
}

/*
After 202300 * 131 + 65 steps, the overall number of visisted steps can be expressed
as a linear combination of the number of visited tiles with respect to the initial map.

The code is generally slow (4sec) because, despite exclusively operating on the initial map, 
the iteration process is slow when stored in a HashMap
*/
fn compute(source : &HashMap<(isize, isize), TileType>, l: isize, n: usize) -> usize {
    let p = HashMap::from([
        // e.g. with respect to the initial map, compute 130 steps where the starting position is north
        ("N", explore(setup(&source, l, (0, l / 2)), l, 130)),
        ("NW", explore(setup(&source, l, (0, 0)), l, 64)),
        ("NW2", explore(setup(&source, l, (0, 0)), l, 65 + 130)),
        ("NE", explore(setup(&source, l, (0, l-1)), l, 64)),
        ("NE2", explore(setup(&source, l, (0, l-1)), l, 65 + 130)),
        ("W", explore(setup(&source, l, (l / 2, 0)), l, 130)),
        ("SW", explore(setup(&source, l, (l-1, 0)), l, 64)),
        ("SW2", explore(setup(&source, l, (l-1, 0)), l, 65 + 130)),
        ("S", explore(setup(&source, l, (l-1, l/2)), l, 130)),
        ("SE", explore(setup(&source, l, (l-1, l-1)), l, 64)),
        ("SE2", explore(setup(&source, l, (l-1, l-1)), l, 65 + 130)),
        ("E", explore(setup(&source, l, (l/2, l-1)), l, 130)),
        ("O", explore(setup(&source, l, (l/2, l/2)), l, 65 + 131)),
        ("O2", explore(setup(&source, l, (l/2, l/2)), l, 65 + 131 + 1)),
    ]);
    
    p["O"] * n*n + p["O2"] * (n-1)*(n-1) + p["S"] + p["W"] + p["N"] + p["E"] 
        + p["NW"] * n + p["NW2"] * (n - 1)
        + p["SW"] * n + p["SW2"] * (n - 1)
        + p["SE"] * n + p["SE2"] * (n - 1)
        + p["NE"] * n + p["NE2"] * (n - 1)
}

fn main() {
    let (map, l) = parse();
    {
        let mut map = map.clone();
        for _ in 1..64 + 1 {
            step(&mut map, l);
        }
        let s1 = map.values().filter(|&&t| t == TileType::Marked).count();
        println!("{}", s1);
    }
    {
        let s2 = compute(&map, l, 202300);
        println!("{}", s2);

    }
}