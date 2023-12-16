use std::{fs, vec};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dirs {
    NORTH, SOUTH, WEST, EAST
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Empty(u16, u16), // .
    Horizontal(u16, u16), // -
    Vertical(u16, u16), // |
    AngledL(u16, u16), // \
    AngledR(u16, u16), // /
    Wall(u16, u16) // '\n'
}

fn run(mut layout : Vec<Mirror>, mut beams : Vec<(isize, Dirs)>, width : isize) -> u32 {
    while !beams.is_empty() {
        let mut append_beams : Vec<(isize, Dirs)> = vec![];
        let mut remove_beams : Vec<usize> = vec![];

        for (i, (mpos, mdir)) in beams.iter_mut().enumerate() {
            let (p, q) = layout.get(*mpos as usize).map(|m| {
                match m {
                    Mirror::Empty(h, v) => (*h, *v),
                    _ => (0, 0)              
                }
            }).unwrap_or((0,0));
            if (p > 1 && (*mdir == Dirs::EAST || *mdir == Dirs::WEST)) || 
                    (q > 1 && (*mdir == Dirs::NORTH || *mdir == Dirs::SOUTH))            
                { remove_beams.push(i); continue; }

            match (layout.get_mut(*mpos as usize), &mdir) {
                (Some(Mirror::Empty(_, v)), Dirs::NORTH) 
                    => { *v += 1; *mpos -= width; }
                (Some(Mirror::Empty(h, _)), Dirs::WEST) 
                    => { *h += 1; *mpos -= 1; }
                (Some(Mirror::Empty(_, v)), Dirs::SOUTH)
                    => { *v += 1; *mpos += width; }
                (Some(Mirror::Empty(h, _)), Dirs::EAST) 
                    => { *h += 1; *mpos += 1; }

                (Some(Mirror::Vertical(h, _)), Dirs::EAST | Dirs::WEST) => { 
                    *h += 1;
                    append_beams.push((*mpos + width, Dirs::SOUTH));
                    *mpos -= width; *mdir = Dirs::NORTH;
                }
                (Some(Mirror::Vertical(_, v)), Dirs::NORTH) 
                    => { *v += 1; *mpos -= width; }
                (Some(Mirror::Vertical(_, v)), Dirs::SOUTH)
                    => { *v += 1; *mpos += width; }

                (Some(Mirror::Horizontal(_, v)), Dirs::NORTH | Dirs::SOUTH) => 
                {
                    *v += 1;
                    append_beams.push((*mpos - 1, Dirs::WEST));
                    *mpos += 1; *mdir = Dirs::EAST;
                } 
                (Some(Mirror::Horizontal(h, _)), Dirs::EAST) => 
                    { *h += 1; *mpos += 1; } 
                (Some(Mirror::Horizontal(h, _)), Dirs::WEST) => 
                    { *h += 1; *mpos -= 1; } 
                // \
                (Some(Mirror::AngledL(_, v)), Dirs::SOUTH) => 
                    { *v += 1; *mdir = Dirs::EAST; *mpos += 1; }
                (Some(Mirror::AngledL(h, _)), Dirs::EAST) => 
                    { *h += 1; *mdir = Dirs::SOUTH; *mpos += width; }
                (Some(Mirror::AngledL(h, _)), Dirs::WEST) => 
                    { *h += 1; *mdir = Dirs::NORTH; *mpos -= width; }
                (Some(Mirror::AngledL(_, v)), Dirs::NORTH) => 
                    { *v += 1; *mdir = Dirs::WEST; *mpos -= 1; }
                
                // /
                (Some(Mirror::AngledR(_, v)), Dirs::NORTH) => 
                    { *v += 1; *mdir = Dirs::EAST; *mpos += 1; }
                (Some(Mirror::AngledR(_, v)), Dirs::SOUTH) => 
                    { *v += 1; *mdir = Dirs::WEST; *mpos -= 1; }
                (Some(Mirror::AngledR(h, _)), Dirs::WEST) => 
                    { *h += 1; *mdir = Dirs::SOUTH; *mpos += width; }
                (Some(Mirror::AngledR(h, _)), Dirs::EAST) => 
                    { *h += 1; *mdir = Dirs::NORTH; *mpos -= width; }

                _ => { remove_beams.push(i); }
            }
            
        }
        beams.extend(append_beams.iter());
        for i in remove_beams.iter().rev() 
            { beams.remove(*i); }

    }
    
    layout.iter().map(|m| {
        match m {
            Mirror::Empty(h, v) => (*h + *v).min(1) as u32,
            Mirror::Horizontal(h, v) => (*h + *v).min(1) as u32,
            Mirror::Vertical(h, v) => (*h + *v).min(1) as u32,
            Mirror::AngledL(h, v) => (*h + *v).min(1) as u32,
            Mirror::AngledR(h, v) => (*h + *v).min(1) as u32,
            Mirror::Wall(_, _) => 0                
        }
    }).sum()
}

fn main () {
    let layout_string = fs::read_to_string("./inputs/input16.txt").expect("Unable to read file");
    let width = (layout_string.chars().position(|c| c == '\n').unwrap() + 1) as isize;
    let layout_orig : Vec<Mirror> = layout_string.chars()
        .map(|c| 
            match c {
                '.' => Mirror::Empty(0, 0),
                '-' => Mirror::Horizontal(0, 0),
                '|' => Mirror::Vertical(0, 0),
                '\\' => Mirror::AngledL(0, 0),
                '/' => Mirror::AngledR(0, 0),
                _ => Mirror::Wall(0, 0)
            })
        .collect_vec();
    {
        let layout = layout_orig.clone();
        let beams : Vec<(isize, Dirs)> = vec![(0, Dirs::EAST)];
        println!("{}", run(layout, beams, width));
    }
    {
        let tt = (0..width-1).map(|i| vec![(i, Dirs::SOUTH)])
            .chain((0..width-1).map(|i| vec![(width * i, Dirs::EAST)]))
            .chain((0..width-1).map(|i| vec![(width * i + width - 2, Dirs::WEST)]))
            .chain((0..width-1).map(|i| vec![((width  - 1) * (width - 1) + i - 1, Dirs::NORTH)]));
        let s2 = tt.map(|b| run(layout_orig.clone(), b, width)).max().unwrap();
        println!("{}", s2);
    }
}