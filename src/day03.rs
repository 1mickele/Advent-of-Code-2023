use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

fn parse(str: String) -> HashMap<(usize, usize), (char, Vec<u32>)> {
    let mut res: HashMap<(usize, usize), (char, Vec<u32>)> = HashMap::new();

    let mut numbers: HashMap<(usize, usize), (i32, usize)> = HashMap::new();
    let mut gears: HashMap<(usize, usize), char> = HashMap::new();

    for (i, s) in str.split('\n').enumerate() {
        let mut j = 0;
        while j < s.len() {
            if s.as_bytes()[j].is_ascii_digit() {
                let mut je = j + 1;
                while je < 140 && s.as_bytes()[je].is_ascii_digit()
                    { je += 1; }
                numbers.insert((i, j), (s[j..je].parse().unwrap(), je - j));
                j = je;
            }
            else if s.as_bytes()[j] != b'.' {
                gears.insert((i, j), s.as_bytes()[j] as char);
                j += 1;
            } else {
                j += 1;
            }
        }
    }

    for ((i, j), (n, l)) in numbers.iter() {
        let neighbour : [(i32, i32); 8] = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),          (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        'outer : for j2 in *j..*j+l {
            for (is, js) in neighbour.iter() {
                let i2 = (*i as i32 + is) as usize;
                let j2 = (j2 as i32 + js) as usize;

                match gears.get(&(i2, j2)) {
                    Some(c) => {
                        let val = match res.get_mut(&(i2, j2)) {
                            Some(val) => &mut val.1,
                            None => {
                                res.insert((i2, j2), (*c, Vec::new()));
                                &mut res.get_mut(&(i2, j2)).unwrap().1
                            }
                        };
                        val.push(*n as u32);
                        break 'outer;
                    },
                    _ => { } 
                }
            }
        }
    }

    // dbg!(&res);

    res
}

fn main() -> io::Result<()> {
    {
        let file = File::open("./inputs/input3.txt")?;
        let mut reader = BufReader::new(&file);
        let mut str = String::new();
        reader.read_to_string(&mut str).expect("cannot read string");

        let res = parse(str);
        let mut s1 = 0;
        for ((_, _), (_, vec)) in res.iter() {
            s1 += vec.iter().sum::<u32>();
        }
        println!("{}", s1);
    }
    {
        let file = File::open("./inputs/input3.txt")?;
        let mut reader = BufReader::new(&file);
        let mut str = String::new();
        reader.read_to_string(&mut str).expect("cannot read string");

        let res = parse(str);
        let mut s1 = 0;
        for ((_, _), (c, vec)) in res.iter() {
            if *c == '*' && vec.len() >= 2 {
                s1 += vec.iter().product::<u32>();
            }
        }
        println!("{}", s1); 
    }

    Ok(())
}