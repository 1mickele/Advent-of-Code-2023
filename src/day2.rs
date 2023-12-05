use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn parse_draw(idx: i32, str: &str) -> [i32; 4] {
    let m = HashMap::from([
        ("red", 0),
        ("green", 1),
        ("blue", 2)
    ]);

    let a = str.split(',')
        .map(|s| s.trim().split(' '))
        .map(|t| t.collect::<Vec<&str>>());

    let mut tuple = [idx, 0, 0, 0];

    for t in a {
        tuple[1 + m[t[1]]] = t[0].parse().unwrap();
    }

    tuple
}

fn parse(str: String) -> Vec<[i32; 4]> {
    let i = &str.chars().position(|c| c == ' ').unwrap() + 1;
    let j = (&str).chars().position(|c| c == ':').unwrap();
    let n = (str[i..j]).parse::<i32>().unwrap();

    let games : Vec<[i32; 4]> = str[j+1..].split(';')
        .map(|s| parse_draw(n, s)).collect();

    return games;
}

fn collect_color(index: usize, game: &Vec<[i32; 4]>) -> i32 {
    return game.iter().map(|d| d[index])
        .fold(0, |a,b| a.max(b));
}

fn main() -> io::Result<()> {
    {
        let file = File::open("./inputs/input2.txt")?;
        let reader = BufReader::new(&file);

        let games : Vec<Vec<[i32; 4]>> = reader.lines()
            .map(|s| parse(s.unwrap()))
            .collect();
        
        let mut partial = 0;

        for game in games {
            if game.iter().all(
                |t| (t[1] <= 12) && (t[2] <= 13) && (t[3] <= 14)
            ) {
                partial += game[0][0];
            }
        }
        
        println!("{}", partial);
    }
    {
        let file = File::open("./inputs/input2.txt")?;
        let reader = BufReader::new(&file);

        let games : Vec<Vec<[i32; 4]>> = reader.lines()
            .map(|s| parse(s.unwrap()))
            .collect();
        
        let mut partial = 0;

        for game in games {
            let r = collect_color(1, &game);
            let g = collect_color(2, &game);
            let b = collect_color(3, &game);
            partial += r*g*b;
        }
        
        println!("{}", partial);
    }

    return Ok(())
}
