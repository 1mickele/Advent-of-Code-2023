use std::{fs, collections::HashMap, vec};
use grid::*;
use itertools::Itertools;
// use priority_queue::PriorityQueue;
// use rand::{Rng, thread_rng};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileType {
    Path,
    Rock,
    Slope((isize, isize)),
    Crossroad
}

fn parse() -> Grid::<TileType> {
    let raw_string = fs::read_to_string("./inputs/input23.txt")
        .expect("Unable to read file"); // // TODO: adjust end node

    let l = raw_string.chars().position(|c| c == '\n').unwrap();
    Grid::from_vec(raw_string.chars()
        .filter(|&c| c != '\n')
        .map(|c| {
            match c {
                '^' => TileType::Slope((-1, 0)),
                '<' => TileType::Slope((0, -1)),
                '>' => TileType::Slope((0, 1)),
                'v' => TileType::Slope((1, 0)),
                '.' => TileType::Path,
                _ => TileType::Rock
            }
        })
        .collect_vec(), l)
}

#[derive(Debug, Clone, PartialEq, Eq)]

struct Path1 {
    steps: usize,
    position: (usize, usize),
    direction: (isize, isize),
    visited: Vec<(usize, usize)>,
}

fn traverse1(chart: &Grid<TileType>) -> usize {
    let mut s1 = 0_usize;
    let mut paths : Vec<Path1> = vec![Path1{
        steps: 0,
        position: (0_usize, 1_usize),
        direction: (1_isize, 0_isize),
        visited: Vec::new()
    }];
    while let Some(mut p) = paths.pop() {
        p.steps += 1;
        p.position = (
            (p.position.0 as isize + p.direction.0) as usize, 
            (p.position.1 as isize + p.direction.1) as usize
        );
        if p.visited.contains(&p.position) { continue; }
        let t = [(1_isize, 0_isize), (-1, 0), (0, -1), (0, 1)].into_iter()
            .filter(|(di, dj)| {
                if p.direction.0 == -di && p.direction.1 == -dj 
                    { return false; }
                match chart.get(p.position.0 as isize + di, 
                    p.position.1 as isize + dj) 
                {
                    Some(TileType::Slope((da, db))) => da == di && db == dj,
                    Some(TileType::Path) => true,
                    _ => false 
                }                
            }).collect_vec();
        
        match t.len() {
            0 => { // dead-end
                if chart.rows() - 1 == p.position.0 { s1 = s1.max(p.steps); }
            },
            1 => { // turn
                p.direction = *t.first().unwrap();
                paths.push(p);
            },
            _ => { // cross-road
                p.visited.push(p.position);
                for (a, b) in t {
                    let mut np = p.clone();
                    np.direction = (a, b);
                    paths.push(np);
                }
            }
        }
    }

    s1
}

// 

type Vertex = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path2 {
    steps: usize,
    position: (usize, usize),
    direction: (isize, isize),
    origin: Vertex,
}

fn build_graph(chart: &mut Grid<TileType>) -> Vec<Vec<(usize, usize)>>
    // Vec<Vertex>, HashMap<Vertex, Vec<(Vertex, usize)>>
{
    let start = (0_usize, 1_usize);
    let end = (140_usize, 139_usize);
    // let end = (22_usize, 21_usize);

    let mut vertices : Vec<Vertex> = vec![start, end];
    let mut edges : HashMap<Vertex, Vec<(Vertex, usize)>> = HashMap::new();
    let mut arcs : Vec<Path2> = vec![Path2{
        steps: 0,
        position: (0_usize, 1_usize),
        direction: (1_isize, 0_isize),
        origin: (0_usize, 1_usize)
    }];

    *chart.get_mut(0_usize, 1_usize).unwrap() = TileType::Rock;

    while let Some(mut arc) = arcs.pop() {
        loop {
            arc.steps += 1;
            arc.position = (
                (arc.position.0 as isize + arc.direction.0) as usize, 
                (arc.position.1 as isize + arc.direction.1) as usize
            );

            if chart.get(arc.position.0, arc.position.1) == Some(&TileType::Crossroad) {
                let v = arc.position;
                edges.entry(arc.origin).or_insert(Vec::new()).push((v, arc.steps));
                edges.entry(v).or_insert(Vec::new()).push((arc.origin, arc.steps));
                break;
            }

            *chart.get_mut(arc.position.0, arc.position.1)
                .unwrap() = TileType::Rock; // mark current position as visited
    
            let roads = [(1_isize, 0_isize), (-1, 0), (0, -1), (0, 1)].into_iter()
                .filter(|(di, dj)| {
                    if arc.direction.0 == -di && arc.direction.1 == -dj 
                        { return false; }
                    match chart.get(arc.position.0 as isize + di, 
                        arc.position.1 as isize + dj) 
                    {
                        Some(TileType::Path) | Some(TileType::Slope(_)) 
                            | Some(TileType::Crossroad) => true,
                        _ => false
                    }                
                }).collect_vec();
    
            match roads.len() {
                0 => { // dead-end
                    let v = arc.position;
                    if vertices.contains(&v) {
                        edges.entry(arc.origin).or_insert(Vec::new()).push((v, arc.steps));
                        edges.entry(v).or_insert(Vec::new()).push((arc.origin, arc.steps));
                    }
                    break;
                }
    
                1 => { // turn
                    arc.direction = *roads.first().unwrap();
                },
    
                _ => { // cross-road
                    let v = arc.position;
                    vertices.push(v);
                    *chart.get_mut(arc.position.0, arc.position.1)
                        .unwrap() = TileType::Crossroad;
                    edges.entry(arc.origin).or_insert(Vec::new()).push((v, arc.steps));
                    edges.entry(v).or_insert(Vec::new()).push((arc.origin, arc.steps));

                    for (a, b) in roads {
                        arcs.push(Path2{
                            origin: v,
                            direction: (a, b),
                            steps: 0,
                            position: v
                        });
                    }
                    break;
                }
            }
        }
    }

    let nedges = edges.iter().map(|(u, vs)| {
            let a = vertices.iter().position(|x| x == u).unwrap();
            let b = vs.iter().map(|(v, c)| 
                    (vertices.iter().position(|x| x == v).unwrap(), *c))
                .collect_vec();
            (a, b)
        })
        .sorted_by(|a, b| { Ord::cmp(&a.0, &b.0) })
        .map(|(_, vs)| vs ).collect_vec();

    nedges
}

/*
// Randomized greedy algorithm using the distance to the end node as the euristic
// does not perform well

// run dijkstra to compute distance to the end node
fn distance_end(v: &Vec<Vertex>, 
    e: &HashMap<Vertex, Vec<(Vertex, usize)>>
) -> HashMap<Vertex, usize> {
    let end = (22_usize, 21_usize);
    // let end = (140_usize, 139_usize);

    let mut dist : HashMap<Vertex, usize> = HashMap::from_iter(
        v.iter().map(|v| (v.clone(), usize::MAX)));
    let mut queue : PriorityQueue<Vertex, isize> = PriorityQueue::from(vec![(end, 0)]);
    
    while let Some((v, _)) = queue.pop() {
        for (u, c) in &e[&v] {
            let d = *c + dist[&v];
            if dist[&u] > d {
                *dist.get_mut(&u).unwrap() = d;
                queue.push(*u, -(d as isize));
            }
        }
    }

    dist
}

fn traverse2(chart: &mut Grid<TileType>) -> usize {
    let start = (0_usize, 1_usize);
    // let end = (22_usize, 21_usize);
    let end = (140_usize, 139_usize);

    let (vertices, edges) = build_graph(chart);
    // dbg!(&v, &e, v.len(), e.len());
    
    let distances = distance_end(&vertices, &edges);
    dbg!(&distances);

    let mut s2 = 0_usize;

    for _ in 0..1000 {
        let mut boundary : PriorityQueue<Vertex, usize> = PriorityQueue::from(
            edges[&start].iter().map(|(u, _)| 
                (u.clone(), rand::thread_rng().gen_range(0..10))).collect_vec()
        );
        let mut longest_path : HashMap<Vertex, usize> = HashMap::from([(start, 0)]);

        while let Some((v, _)) = boundary.pop() {
            let (_, d) = edges[&v].iter()
                .filter(|(u, _)| longest_path.keys().contains(u))
                .map(|(u, c)| (u, longest_path[u] + c))
                .max_by_key(|(_, c)| *c).unwrap();

            longest_path.insert(v, d);
            if v == end { break; }
            boundary.extend(edges[&v].iter()
                .filter(|(u, _)| !longest_path.keys().contains(u))
                .map(|(u, c)| (u.clone(), 
                    if *u != end { rand::thread_rng().gen_range(1..*c + 100) } else { 0 }
                )));
        }
        
        s2 = s2.max(longest_path[&end]);
    }

    // dbg!(&edges[&(9, 103)]);

    s2
}
*/

struct GraphPath {
    steps: usize,
    current: usize,
    visited: Vec<bool>,
}

// takes ~2 secs (9sec shaved) 
fn traverse_force(
    edges: &Vec<Vec<(usize, usize)>>, 
    available: Vec<bool>,
    start: usize,
    end: usize
) -> usize {
    let mut s2 : usize = 0;
    
    let mut paths : Vec<GraphPath> = vec![GraphPath{
        steps: 0, current: start, visited: available
    }];

    while let Some(path) = paths.pop() {
        if path.current == end 
            { s2 = s2.max(path.steps); continue; }

        for (v, c) in &edges[path.current] {
            if !path.visited[*v] {
                let mut t = path.visited.clone();
                t[*v] = true;
                paths.push(GraphPath{
                    steps: c + path.steps, current: v.clone(), visited: t
                })
            }
        }
    }

    s2
}

fn main() {
    let mut chart = parse();
    {
        let s1 = traverse1(&chart);
        println!("{}", s1);
    }
    {
        let edges = build_graph(&mut chart);

        let mut v = vec![false; edges.len()];
        *v.first_mut().unwrap() = true;

        let s2 = traverse_force(&edges, v, 0_usize, 1_usize);
        println!("{}", s2);
    }
}

/*
// Failed attempts: 
// - small perturbation of the shortest path
// - random perturbation of a random path based on distance-to-end heuristic
// - random local bruteforcing starting from a random path

    // .. main
    {        
        let edges = build_graph(&mut chart);
        let s2 = (1..1000).map(|_| traverse3(&edges)).max().unwrap();
        println!("{}", s2);
    }

fn shortest_path(edges: &Vec<Vec<(usize, usize)>>) 
    -> Vec<usize>
{
    let mut dist_and_origin: Vec<(usize, usize)> = vec![(usize::MAX, 0); edges.len()];
    let mut queue : PriorityQueue<usize, isize> = PriorityQueue::from(vec![(0, 0)]);

    while let Some((v, _)) = queue.pop() {
        let (dv, _) = dist_and_origin[v];
        for (u, c) in &edges[v] {
            let (du, ogu) = dist_and_origin.get_mut(*u).unwrap();
            if *du > *c + dv {
                (*du, *ogu) = (*c + dv, v);
                queue.push(*u, -(*du as isize));
            }
        }

        if v == 1 { break; } // the end was reached
    }

    let mut path : Vec<usize> = vec![1];
    while path.last() != Some(&0_usize) {
        path.push(dist_and_origin[*path.last().unwrap()].1);
    }

    path.reverse();
    path
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphPath2 {
    steps: usize,
    current: usize,
    visited: Vec<bool>, // now it is path_forceused as cache
    vertices: Vec<usize>
}

fn path_force(
    edges: &Vec<Vec<(usize, usize)>>, 
    available: Vec<bool>,
    start: usize,
    end: usize,
) -> Vec<usize> {  
    let mut vertices : Vec<usize> = Vec::new();
    let mut paths : Vec<GraphPath2> = vec![GraphPath2{
        steps: 0, current: start, visited: available, vertices: vec![start]
    }];

    while let Some(path) = paths.pop() {
        if path.current == end { 
            vertices = path.vertices;
            continue;
        }
        for (v, c) in &edges[path.current] {
            if !path.visited[*v] {
                let mut t = path.visited.clone();
                t[*v] = true;
                let mut vx = path.vertices.clone();
                vx.push(*v);

                paths.push(GraphPath2{
                    steps: c + path.steps, current: v.clone(), 
                    visited: t, vertices: vx
                })
            }
        }
    }

    vertices
}

fn path_random(
    edges: &Vec<Vec<(usize, usize)>>
) -> Vec<usize> {  
    let mut bitmask = vec![false; edges.len()];
    bitmask[0] = true;
    let mut paths : PriorityQueue<GraphPath2, isize> = PriorityQueue::from(
        vec![(GraphPath2{ 
            steps: 0, current: 0, visited: bitmask, vertices: vec![0]
        }, 0)]);

    while let Some((path, _)) = paths.pop() {
        if path.current == 1 { 
            return path.vertices;
        }
        for (v, c) in &edges[path.current] {
            if !path.visited[*v] {
                let mut vs = path.vertices.clone();
                vs.push(*v);
                let mut vvs = path.visited.clone();
                vvs[*v] = true;
                paths.push(GraphPath2{
                    steps: c + path.steps, current: v.clone(), 
                    visited: vvs, vertices: vs
                }, thread_rng().gen_range(0..100));
            }
        }
    }

    Vec::new()
}

// starting from a random path, use locally bruteforcing to find the maximal path
fn traverse3(edges: &Vec<Vec<(usize, usize)>>) -> usize {    
    let mut path = path_random(&edges);
    
    let mut vertex = vec![false; edges.len()];
    for i in &path 
        { vertex[*i] = true; }

    for _ in 0..10 {
        let n = thread_rng().gen_range(1..path.len() / 2); // n# of nodes to remove
        let i = thread_rng().gen_range(2..path.len() - path.len() / 2); // where to remove
        let a = path[i];
        let b = path[i + n];

        for _ in i + 1..i + n {
            vertex[path[i+1]] = false;
            path.remove(i + 1);
        }
        vertex[b] = false;

        for v in path_force(&edges, vertex.clone(), a,b).iter()
            .skip(1).rev().skip(1) 
        {
            path.insert(i+1, *v);
            vertex[*v] = true;
        }
        vertex[b] = true;
    }

    let t : usize = path.iter().tuple_windows().map(|(u, v)| {
        edges[*u].iter().find(|x| &x.0 == v)
            .map(|(_, c)| c).unwrap()
    }).sum::<usize>();

    // dbg!(&path);
    dbg!(t);

    t
}

*/