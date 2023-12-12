use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dirs {
    NORTH,
    SOUTH,
    WEST,
    EAST
}

fn parse() -> (
    Vec<(u8, bool)>, // map
    usize,  
    usize,
    Dirs
)
{
    let raw_data = *std::include_bytes!("../inputs/input10.txt");

    (raw_data.map(|c| (c, false)).to_vec(), 
        raw_data.iter().position(|c| *c == b'\n').unwrap(), // width of map
        raw_data.iter().position(|c| c == &b'S').unwrap(), // starting position
        Dirs::SOUTH) // hard-coded
}

fn compute_table() -> Vec<(u8, Dirs, Dirs)> {
    fn flip(s : &Dirs) -> Dirs {
        match s {
            Dirs::NORTH => Dirs::SOUTH,
            Dirs::SOUTH => Dirs::NORTH,            
            Dirs::EAST => Dirs::WEST,
            Dirs::WEST => Dirs::EAST
        }
    }
    
    let pipes_rules = [
        (b'|', Dirs::NORTH, Dirs::SOUTH),
        (b'-', Dirs::EAST, Dirs::WEST),
        (b'7', Dirs::WEST, Dirs::SOUTH),
        (b'J', Dirs::WEST, Dirs::NORTH),
        (b'F', Dirs::SOUTH, Dirs::EAST),
        (b'L', Dirs::EAST, Dirs::NORTH)
    ];

    pipes_rules.into_iter().map(|(c, a, b)| {
            [(c, flip(&a), b), (c, flip(&b), a)]
        })
        .flatten()
        .collect::<Vec<(u8, Dirs, Dirs)>>()
}
fn main() {
    
    let (mut map, width, mut pos, mut dir) = parse();
    map.get_mut(pos).unwrap().0 = b'|'; // hard-coded map patching ahah

    let mut s1 = 0;
    let step_table = compute_table();
    loop {
        pos = match dir {
            Dirs::NORTH => pos - 1 - width,
            Dirs::SOUTH => pos + 1 + width,            
            Dirs::EAST => pos + 1,
            Dirs::WEST => pos - 1
        };

        let nc = map.get_mut(pos).unwrap();
        if nc.1 { break; }
        s1 += 1; 
        nc.1 = true; // marks that this bit is part of the loop

        dir = step_table.iter()
            .find(|(c,s, _) | {
                (nc.0 == *c) && (*s == dir)
            }).unwrap().2;
    }
    println!("{}", s1 / 2);

    let mut s2 = 0;
    let mut j = 0;
    while j < map.len() {
        let mut crossings = 0;
        while map.get(j).map_or(false, |c| c.0 != b'\n') {
            let nc = map.get(j).unwrap();
            match nc {
                (b'|', true) => { crossings += 1; j += 1; }
                (b'F', true) | (b'L', true) => {
                    let (jp, (c, _)) = map[j + 1..].iter()
                        .find_position(|(c, _)| {
                            *c == b'J' || *c == b'7'
                        }).unwrap();
                    if (nc.0 == b'F' && *c == b'J') | (nc.0 == b'L' && *c == b'7') 
                        { crossings += 1; }
                    j += jp + 2;
                }
                _ => {
                    if crossings % 2 == 1 { s2 += 1; }
                    j += 1; 
                }
            }
        }
        j += 1;
    }

    println!("{}", s2);
}