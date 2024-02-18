use std::{fs, path::Path, vec};

use itertools::Itertools;

fn parse(filename: impl AsRef<Path>) -> Vec<(Vec<u8>, Vec<usize>)> {
    let raw_string = fs::read_to_string(filename).expect("Unable to read file");
    raw_string.split('\n').map(|l| {
        let ws = l.chars().position(|c| c == ' ').unwrap();
        (l[0..ws].as_bytes().to_vec(), 
            l[ws+1..].split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect())
    }).collect()
}

/*
// recursive non memoized solution
fn backtrack(
    (str, data) : (&Vec<u8>, &Vec<usize>), 
    (str_index, data_index): (usize, usize),
) -> usize 
{
    if str_index == str.len() 
        { return if data_index == data.len() { 1 } else { 0 }; }
    
    if data_index == data.len()
        { return if str[str_index..].iter().all(|&x| x != b'#')
            { 1 } else { 0 }; 
        }

    let mut res: usize = 0;
    let c: u8 = str[str_index];

    if c == b'.' || c == b'?' {
        res += backtrack((str, data), (str_index + 1, data_index));
    }

    if c == b'#' || c == b'?' {
        let d = data[data_index];


        if d <= str.len() - str_index &&
            str[str_index..str_index+d].iter().all(|&x| x != b'.')
        {
            if str_index + d == str.len() {
                res += backtrack((str, data), (str_index + d, data_index + 1));
            } else if str[str_index + d] != b'#' {
                res += backtrack((str, data), (str_index + d + 1, data_index + 1));
            }
        }
    }

    res
}
*/

fn dp(str: &Vec<u8>, data: &Vec<usize>) -> usize {
    let (n, m) = (str.len(), data.len());
    let mut table: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];

    for i in (0..n+1).rev() {
        for j in (0..m+1).rev() {
            if i == str.len() {
                table[i][j] = if j == data.len() { 1 } else { 0 };
                continue;
            }

            if j == data.len() {
                table[i][j] = if str[i..].iter().all(|&x| x != b'#')
                    { 1 } else { 0 }; 
                continue;
            }

            let mut res: usize = 0;
            let c: u8 = str[i];
        
            if c == b'.' || c == b'?' {
                res += table[i+1][j];
            }
        
            if c == b'#' || c == b'?' {
                let d = data[j];
                if d <= str.len() - i &&
                    str[i..i+d].iter().all(|&x| x != b'.')
                {
                    if i + d == str.len() {
                        res += table[i + d][j + 1];
                    } else if str[i + d] != b'#' {
                        res += table[i + d + 1][j + 1];
                    }
                }
            }

            table[i][j] = res;
        }
    }

    return table[0][0];
}

fn main() {
    let records = parse("./inputs/input12.txt");
    {
        let s1 : usize = records.iter().map(|(s, d)| {
            dp(s, d)
        }).sum();

        println!("{}", s1); 
    }
    {
        let records = records.iter().map(|e| {
            let mut s: Vec<u8> = e.0.clone();
            s.push(b'?'); s = s.repeat(5); s.pop();
            (s, e.1.repeat(5))
        }).collect_vec();

        let s2 : usize = records.iter().map(|(s, d)| {
            dp(s, d)
        }).sum();

        println!("{}", s2);         
    }
}