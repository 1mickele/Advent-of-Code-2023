use std::{fs, usize, isize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn parse () -> Vec<Vec<usize>>  
{
    let raw_string = fs::read_to_string("./inputs/input17.txt").expect("Unable to read file");
    raw_string.split('\n').map(|l| 
        l.as_bytes().iter().map(|c|
            c.abs_diff(b'0') as usize, // heat-cost of single tile
        ).collect()
    ).collect()
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    position: (usize, usize),
    index: usize,
    heat_fcost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_fcost.cmp(&self.heat_fcost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_path (map : &Vec<Vec<usize>>, min_step: usize, max_step: usize, keep_size: usize) -> usize {    
    let width = map.len() as isize;
    let h = |i: isize, j: isize| -> usize 
        { 3 * (width.abs_diff(i) + width.abs_diff(j)) };

    type Path = (usize, (isize, isize), usize);
    let mut paths: Vec<Vec<Vec<Path>>> = Vec::from_iter(
            (0..width).map(|_| (0..width).map(|_| 
                vec![(usize::MAX, (0_isize, 0_isize), 0_usize)]               
            ).collect())
        );
    
    paths[0][0] = vec![(0_usize, (0_isize, 1_isize), 0_usize),
        ((0_usize, (1_isize, 0_isize), 0_usize))];
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State { 
        position: (0, 0), 
        heat_fcost: h(0, 0),
        index: 0
    });
    heap.push(State { 
        position: (0, 0), 
        heat_fcost: h(0, 0),
        index: 1
    });

    while let Some(State { 
        position: (i, j),
        index: n, 
        heat_fcost: _,
    }) = heap.pop() 
    {
        let (c, (u, v), s) = paths[i][j][n];
        'outer: for (a, b) in [(1_isize, 0_isize), (-1, 0), (0, 1), (0, -1)]
                .iter().filter(|(a, b)| 
                    (a.abs_diff(-u) + b.abs_diff(-v)) > 0)
        {
            // if i == width as usize - 1 && j == width as usize - 1
            //    { break; }

            if *a + (i as isize) < 0 || *a + (i as isize) >= width 
                || *b + (j as isize) < 0 || *b + (j as isize) >= width
                { continue; }

            if s < min_step && (*a, *b) != (u, v)
                { continue; }
            let ss = if *a == u as isize && *b == v as isize
                { if s >= max_step { continue; } else { s + 1 }} else { 1 };
            
            let (ii, jj) = ((*a + i as isize) as usize, (*b + j as isize) as usize);
            
            let new_cost = c + map[ii as usize][jj as usize];

            let (mut max, mut idx) = (usize::MIN, 0_usize);
            for (id, otp) in paths[ii][jj].iter().enumerate() {
                if otp.0 > max {
                    (max, idx) = (otp.0, id);
                }
                if (new_cost, (*a, *b), ss) == *otp {
                    continue 'outer;
                }
            }
            if new_cost > max { continue; }

            if paths[ii][jj].len() > keep_size {
                paths[ii][jj][idx] = (new_cost, (*a, *b), ss);
            } else {
                idx = paths[ii][jj].len();
                paths[ii][jj].push((new_cost, (*a, *b), ss));
            }

            heap.push(State {
                position: (ii, jj),
                index: idx,
                heat_fcost: h(ii as isize, jj as isize) + new_cost
            });
        }
    } 

    paths[width as usize - 1][width as usize - 1].iter()
        .min_by_key(|(c, _, _)| c).unwrap().0
}

fn main () {
    let s1 = find_path(&parse(), 0, 3, 50);
    println!("{}", s1);

    let s2 = find_path(&parse(), 4, 10, 50);
    println!("{}", s2);
}
