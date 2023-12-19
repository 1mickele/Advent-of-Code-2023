use std::{fs, usize, isize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;

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
    // heading: ((isize, isize), usize),
    // expires: u8,
    // came_from: Option<Rc<RefCell<State>>>
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

fn find_path (map : &Vec<Vec<usize>>, min_step: usize, max_step: usize, cache: usize) -> usize {    
    let width = map.len() as isize;
    let h = |i: isize, j: isize| -> usize 
        { 3 * (width.abs_diff(i) + width.abs_diff(j)) };

    // heat-score, heading, n# steps
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

            if paths[ii][jj].len() > cache {
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
            
            /*
            if new_cost < *paths[ii][jj].iter()
                    .map(|(c, _, _)| c).max().unwrap()
                && paths[ii][jj].iter().all(
                    |(c, d, st)| 
                        new_cost != *c || *d != (*a, *b) || *st != ss)
            {
                let mut nn: usize = 0;
                if paths[ii][jj].len() > 10 {
                    nn = paths[ii][jj].iter_mut()
                        .position_max_by_key(|(oc, od, os)| *oc).unwrap();
                    let (oc, od, os) = paths[ii][jj].get_mut(nn).unwrap();
                    *oc = new_cost;
                    *od = (*a, *b);
                    *os = ss;
                } else {
                    nn = paths[ii][jj].len();
                    paths[ii][jj].push((new_cost, (*a, *b), ss));
                }
                heap.push(State {
                    position: (ii, jj),
                    index: nn,
                    heat_fcost: h(ii as isize, jj as isize) + new_cost
                })
            }
            */
        }
    } 

    // dbg!(&paths[0][3]);
    paths[width as usize - 1][width as usize - 1].iter()
        .min_by_key(|(c, _, _)| c).unwrap().0
    // *paths[width as usize - 1][width as usize - 1].unwrap().iter()
    //    .min_by_key(|(a, _, _)| a).unwrap().0
}
/*
fn find_path (map : &Vec<Vec<usize>>) -> usize {    
    let width = map.len();
    let h = |i: usize, j: usize| -> usize 
       { 0_usize.abs_diff(i) + 8_usize.abs_diff(j) };

    let mut cost: Vec<Vec<(usize, usize)>> = Vec::from_iter(
        (0..width).map(|_| (0..width).map(|_| (usize::MAX, usize::MAX)).collect())
    );
    let mut came_from: Vec<Vec<(usize, usize)>> = Vec::from_iter(
        (0..width).map(|_| (0..width).map(|_| (0, 0)).collect())
    );
    cost[0][0] = (0_usize, h(0,0));

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State { 
        position: (0, 0), 
        heading: ((0, 1), 0),
        heat_fcost: h(0, 0),
        expires: u8::MAX,
        came_from: None,
    });

    let mut stash : Vec<State> = Vec::new(); 

    while let Some(state) = heap.pop() {
        let State { 
            position: (i, j), 
            heading: ((u, v), s), 
            heat_fcost: f,
            expires: e,
            came_from: _
        } = state;

        // if i == width - 1 && j == width - 1
        //    { dbg!("AAA", i, j); break; }

        for (a, b) in [(1_isize, 0_isize), (-1, 0), (0, 1), (0, -1)]
                .iter().filter(|(a, b)| 
                    (a.abs_diff(-(u as isize)) + b.abs_diff(-(v as isize))) > 0)
        {
            let (ii, jj) = ((*a + i as isize), 
                (*b + j as isize));

            if ii < 0 || ii >= width || jj < 0 || jj >= width { continue; }
            let ss = if *a == u as isize && *b == v as isize
                { if s >= 3 { continue; } else { s + 1 }} else { 1 };

            let new_cost = cost[i][j].0 + map[ii as usize][jj as usize];
            if cost[ii as usize][jj as usize].0 > new_cost {
                heap.push(State {
                    position: (ii as usize, jj as usize),
                    heading: ((*a, *b), ss),
                    heat_fcost: new_cost + h(ii as usize, jj as usize),
                    expires: 10,
                    came_from: Some(Rc::new(RefCell::new(state.clone()))),
                });
                
                cost[ii as usize][jj as usize] = (new_cost, new_cost + h(ii as usize, jj as usize));
                came_from[ii as usize][jj as usize] = (i, j);
                let st = State {
                    position: (ii as usize, jj as usize),
                    heading: ((*a, *b), ss),
                    heat_fcost: new_cost + h(ii as usize, jj as usize),
                    expires: u8::MAX,
                    came_from: None
                };
                if !&heap.iter().contains(&st)
                    { heap.push(st); }
            }
        }
    }

    let (mut i, mut j) = (width-1, width-1);
    while i!=0 || j != 0 {
        println!("{},{}", i,j);
        (i,j) = came_from[i][j];
    }

    // @## #
    //   ###
    return cost[0][8].0;
}
*/

fn main () {
    let s1 = find_path(&parse(), 0, 3, 50);
    println!("{}", s1);

    let s2 = find_path(&parse(), 4, 10, 50);
    println!("{}", s2);
}
