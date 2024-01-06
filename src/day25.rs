use std::fs;
use std::collections::HashMap;
use queues::*;

use itertools::Itertools;

fn parse() 
    // -> HashMap<String, Vec<String>> {
    -> Vec<Vec<usize>>
{
    let raw_string = fs::read_to_string("./inputs/input25.txt")
        .expect("Unable to read file");

    let mut graph : HashMap<String, Vec<String>> = HashMap::new();
    for l in raw_string.split('\n') {
        let mut it = l.split(": ");
        let s = it.next().unwrap().to_string();
        let es = it.next().unwrap().split(' ')
            .map(|s| s.to_string()).collect_vec();
        graph.entry(s.clone()).or_insert(Vec::new())
            .extend_from_slice(&es[..]);
        for t in es {
            graph.entry(t).or_insert(Vec::new()).push(s.clone());
        }
    }

    // convert to bitmask
    let keys = graph.keys().map(|x| x.clone()).collect_vec();
    let bgraph: Vec<Vec<usize>> = graph.into_iter().map(|(k, v)| {
        (keys.iter().position(|x| **x == k).unwrap(),
            v.iter().map(|x| keys.iter()
                .position(|y| *x == **y).unwrap())
                .collect_vec())
    }).sorted_by_key(|(i,_)| *i)
    .map(|(_, x)| x).collect_vec();

    bgraph
}

fn shortest_path(
    i: usize, j:usize, 
    edges: &Vec<Vec<usize>>, 
    graphmask: &Vec<bool>
) -> Option<Vec<usize>>
{
    let mut comes_from : HashMap<usize, usize> = HashMap::new();
    let mut queue : Queue<usize> = queue![i];
    let mut visited : Vec<bool> = vec![false; edges.len()]; visited[i] = true;

    while let Ok(v) = queue.remove() {
        if v == j {
            // path-found
            let mut path : Vec<usize> = vec![j];
            while path.last() != Some(&i) {
                path.push(comes_from[path.last().unwrap()]);
            }
        
            path.reverse();
            return Some(path);
        }
        for u in &edges[v] {
            if !graphmask[*u] || visited[*u] { continue; }
            comes_from.insert(*u, v);
            queue.add(*u);
            visited[*u] = true;
        }
    }

    None
}

fn find_ways(i: usize, j: usize, edges: &Vec<Vec<usize>>) -> bool {
    let mut graphmask : Vec<bool> = vec![true; edges.len()];
    for _ in 0..3 {
        let path = shortest_path(i, j, &edges, &graphmask);
        if path.is_none() { return false; }
        let path = path.unwrap();
        for v in &path[1..&path.len()-1]
            { graphmask[*v] = false; }
    }
    
    shortest_path(i, j, &edges, &graphmask).is_some()
}

fn main () {
    let edges = parse();
    {
        let mut s1 = 0_usize;
        for j in 1..edges.len() {
            if !find_ways(0, j, &edges) { s1 += 1; }
        }
        println!("{}", edges.len() * s1);
    }
}