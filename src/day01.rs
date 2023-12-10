use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_digit(mut src: impl Iterator<Item = char>) -> u32 {
    src.find_map(|c| c.is_ascii_digit().then_some(c))
        .unwrap().to_digit(10).unwrap()
}

fn get_number(src: Option<String>) -> Option<u32> {
    let a = src.as_ref().map(|s| get_digit(s.chars()));
    let b = src.as_ref().map(|s| get_digit(s.chars().rev()));
    a.and_then(|x| b.map(|y| (x,y))).map(|(x,y)| x*10 + y)
}

fn get_digit2(src: &String, digits: &HashMap<String, u32>) -> Option<u32> {
    // dbg!(src, digits);
    for i in 0..src.len() {
        let c = src.as_bytes()[i];
        if c.is_ascii_digit() {
            return (c as char).to_digit(10);
        }

        for s in digits.keys() {
            if (i + s.len() <= src.len()) && (*s == &src[i..i+s.len()]) {
                // dbg!(s);
                return Some(digits[s]);
            }
        }
    }

    return None;
}

fn get_number2(src: Option<String>) -> Option<u32> {
    let str = src?;

    let items: [(String, u32); 9] = [
        ("one".to_string(), 1), ("two".to_string(), 2), 
        ("three".to_string(), 3), ("four".to_string(), 4), 
        ("five".to_string(), 5), ("six".to_string(), 6), 
        ("seven".to_string(), 7), ("eight".to_string(), 8), 
        ("nine".to_string(), 9)
    ];

    let english_digits : HashMap<String, u32>= HashMap::from(items.clone());

    let english_digits_rev : HashMap<String, u32> = items.iter()
        .map(|(k, v)| ((k.chars().rev().collect(), *v)))
        .collect();

    let a = get_digit2(&str, &english_digits);
    let b = get_digit2(&(str.chars().rev().collect()), 
        &english_digits_rev);

    return Some(a? * 10 + b?)
}
fn main() -> io::Result<()>{
    {
        let file = File::open("./inputs/input1.txt")?;
        let reader = BufReader::new(&file);

        // reader.lines().map(|line| getNumber())
        let s1 = reader.lines()
            .map(|s| get_number(s.ok()))
            .fold(0, |x, y| x + y.unwrap_or(0));

        println!("{}", s1);
    }
    {
        let file = File::open("./inputs/input1.txt")?;
        let reader = BufReader::new(&file);

        // reader.lines().map(|line| getNumber())
        let s2 = reader.lines()
            .map(|s| get_number2(s.ok()))
            .fold(0, |x, y| x + y.unwrap_or(0));

        println!("{}", s2);
    }

    Ok(())
}
