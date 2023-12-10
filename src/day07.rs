use std::{path::Path, fs};
use itertools::Itertools;
use std::collections::HashMap;

type Hand = [u8; 5];

fn lines_from(filename: impl AsRef<Path>, delimiter: &str) -> Vec<String> {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    
    raw_string.split(delimiter)
        .map(|s| s.to_string())
        .collect()
}

fn parse(filename: impl AsRef<Path>, tab: &HashMap<char, u8>) -> Vec<(Hand, u64)> {
    lines_from(filename, "\n").iter()
        .map(|line| -> (Hand, u64) { 
            let p = line.split(' ').collect_tuple::<(_,_)>().unwrap();
            let hand = p.0.chars()
                .map(|c| tab[&c])
                .collect_vec()
                .try_into().unwrap();
            let bid = p.1.parse::<u64>().unwrap();
            (hand, bid)
        })
        .collect()
}

fn get_type(hand: &Hand) -> u8 {
    // 255 here servers as a delimiter to prevent out-of-bound error 
    // while making the algorithm nicer
    let mut h = [
        hand[0], hand[1], hand[2],
        hand[3], hand[4], 255 ];
    h.sort();
    
    
    let mut types = [0, 0, 0, 0]; // n# pairs, n# triple, n# ...

    let mut j = 0;
    for i in 0..h.len() - 1 {
        if h[i] == h[i+1] {
            j += 1;
        } else {
            if j > 0 { 
                types[j - 1] += 1; 
            }
            j = 0;
        }
    }

    match types {
        [0, 0, 0, 0] => return 0, // hight card
        [1, 0, 0, 0] => return 1, // one pair
        [2, 0, 0, 0] => return 2, // two pairs
        [0, 1, 0, 0] => return 3, // three of a kind
        [1, 1, 0, 0] => return 4, // full house
        [0, 0, 1, 0] => return 5, // four of a kind
        [0, 0, 0, 1] => return 6, // five of a kind
        _ => return u8::MAX 
    }
}

fn get_type2(hand: &Hand) -> u8 {
    // 255 here servers as a delimiter to prevent out-of-bound error 
    // while making the algorithm nicer
    let mut njoker = 0;
    let mut h = 
        vec![hand[0], hand[1], hand[2], 
            hand[3], hand[4], 255]
        .into_iter().filter(|x| {
        if *x == 0 {
            njoker += 1;
            return false;
        }
        return true;
    }).collect::<Vec<_>>();
    h.sort();
    
    let mut types = [0, 0, 0, 0]; // n# pairs, n# triple, n# ...

    let mut j = 0;
    for i in 0..h.len() - 1 {
        if h[i] == h[i+1] {
            j += 1;
        } else {
            if j > 0 { 
                types[j - 1] += 1; 
            }
            j = 0;
        }
    }

    // pair (3) -> triple (6) -> four (9) -> five (12)
    // 2 pairs (4) -> full (8)
    match types {
        [0, 0, 0, 0] => {
            if njoker == 5 { return 12; }
            return 0 + 3 * njoker
        } , // hight card
        [1, 0, 0, 0] => return 3 + 3 * njoker, // one pair
        [2, 0, 0, 0] => return 4 + 4 * njoker, // two pairs
        [0, 1, 0, 0] => return 6 + 3 * njoker, // three of a kind
        [1, 1, 0, 0] => return 8, // full house
        [0, 0, 1, 0] => return 9 + 3 * njoker, // four of a kind
        [0, 0, 0, 1] => return 12, // five of a kind
        _ => { return u8::MAX } 
    }
}

fn rank(hand: &Hand) -> u64 {
    let t = get_type(hand) as u64;
    13_u64.pow(6) * t as u64 + hand.iter()
        .enumerate()
        .map(|(i, x)| 13_u64.pow((5 - i - 1) as u32) * *x as u64)
        .sum::<u64>()
}

fn rank2(hand: &Hand) -> u64 {
    let t = get_type2(hand) as u64;
    13_u64.pow(6) * t as u64 + hand.iter()
        .enumerate()
        .map(|(i, x)| 13_u64.pow((5 - i - 1) as u32) * *x as u64)
        .sum::<u64>()
}

fn main() {
    {
        let tab = HashMap::from([
            ('2', 0), ('3', 1), ('4', 2), ('5', 3), ('6', 4), 
            ('7', 5), ('8', 6), ('9', 7), ('T', 8), ('J', 9), 
            ('Q', 10), ('K', 11), ('A', 12)
        ]);
        let hands = parse("./inputs/input7.txt", &tab);
        
        let mut a : Vec<_> = hands.iter()
            .map(|(h, b)| (h, b, rank(h)))
            .collect();
        a.sort_by_key(|(_,_,r)| *r);
        let s1 = a.iter()
            .map(|(_,b,_)| b)
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (i as u64 + 1) * **x);
        println!("{}", s1);
    }
    {
        let tab = HashMap::from([
            ('J', 0), ('2', 1), ('3', 2), ('4', 3), ('5', 4), 
            ('6', 5), ('7', 6), ('8', 7), ('9', 8), ('T', 9),
            ('Q', 10), ('K', 11), ('A', 12)
        ]);
        let hands = parse("./inputs/input7.txt", &tab);
        let mut a : Vec<_> = hands.iter()
            .map(|(h, b)| (h, b, rank2(h)))
            .collect();
        a.sort_by_key(|(_,_,r)| *r);
        let s2 = a.iter()
            .map(|(_,b,_)| b)
            .enumerate()
            .fold(0, |acc, (i, x)| acc + (i as u64 + 1) * **x);
        println!("{}", s2);
    }
}