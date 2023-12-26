use std::fs;
use std::collections::HashMap;

use itertools::Itertools;

fn parse() -> 
    (
        HashMap<String, Vec<(Option<(char, i128)>, String)>>,
        Vec<HashMap<char, i128>>
    )
{
    let raw_string = fs::read_to_string("./inputs/input19.txt").expect("Unable to read file");
    let mut a = raw_string.split("\n\n");
    let l = a.next().map(|b| {
        b.split('\n').map(|l| {
            let i = l.chars().position(|p| p == '{').unwrap();
            (
                l[0..i].to_string(),
                l[i+1..l.len()-1].split(',')
                    .map(|t| {
                        t.chars().position(|p| p == ':').map(|j| { 
                            let c = *t.as_bytes().first().unwrap() as char;
                            let n = match t.as_bytes().get(1) {
                                Some(b'>') => 1,
                                Some(b'<') => -1,
                                _ => 0
                            } * t[2..j].parse::<i128>().unwrap();
                            (Some((c, n)), t[j+1..].to_string())
                        }).unwrap_or((None, t.to_string()))
                    }).collect_vec()
            )
        }).collect()
    }).unwrap();

    let r = a.next().map(|b| {
        b.split('\n').map(|l| {
            l[1..l.len()-1].split(',').map(|q| {
                (
                    q.chars().next().unwrap(),
                    q[2..].parse::<i128>().unwrap()
                )                
            }).collect()
        }).collect_vec()
    }).unwrap();

    (l,r)
}

fn accept(
    entry : &HashMap<char, i128>, 
    rules : &HashMap<String, Vec<(Option<(char, i128)>, String)>>
) -> bool {
    let (mut state, mut index) = (&"in".to_string(), 0_usize);
    while state != "A" && state != "R" {
        // x - |val| == same sign of val => ok
        // dbg!(&rules[state][index], entry, state, '\n');
        if let (Some((c, n)), nstate) = &rules[state][index] {
            if (entry[c] - n.abs()) * n.signum() <= 0 {
                index += 1;
            } else {
                state = &nstate;
                index = 0;
            }
        } else if let (None, nstate) = &rules[state][index] {
            // dbg!(state, nstate, entry, '\n');

            state = &nstate;
            index = 0;        
        }
    }
    state == "A"
}

fn solver(
    curr: &String,
    // curr: &Vec<(Option<(char, i128)>, String)>,
    rules : &HashMap<String, Vec<(Option<(char, i128)>, String)>>,
    mut constraints : HashMap<char, (i128, i128)>
) -> i128 
{
    let mut acc = 0;
    if curr == "A" {
        return constraints.iter()
            .map(|(_, (l, r))| -> i128 { *r - *l })
            .fold(1_i128, |a, x| a * if x > 0 { x } else { 0 });
    } else if curr == "R" {
        return 0;
    }

    for r in rules.get(curr).unwrap() {
        match &r {
        &(Some((c, n)), s) => {
                let mut nconstraints = constraints.clone();
                if let Some((l, r)) = nconstraints.get_mut(&c) {
                    if *n < 0 {
                        *r = (*r).min(n.abs());
                        constraints.get_mut(c).unwrap().0 = *r;
                    } else {
                        *l = (*l).max(*n) + 1;
                        constraints.get_mut(c).unwrap().1 = *l;
                    }
                }
                acc += solver(&s, rules, nconstraints);
            }

            &(None, s) => {
                return acc + solver(&s, rules, constraints.clone());
            }
        }
    }
    
    acc
}

fn main() {
    let (rs, es) = parse();
    {
        let s1 : i128 = es.iter()
            .filter(|e| accept(e, &rs))
            .map(|e| e[&'x'] + e[&'m'] + e[&'a'] + e[&'s'])
            .sum();
        println!("{}", s1);
    }
    {
        let t : HashMap<char, (i128, i128)> = HashMap::from([
            ('x', (1, 4000 + 1)),
            ('m', (1, 4000 + 1)),
            ('a', (1, 4000 + 1)),
            ('s', (1, 4000 + 1)),
        ]); // they read xmas!

        let s2 = solver(&"in".to_string(), &rs, t);
        println!("{}", s2);
    }
}