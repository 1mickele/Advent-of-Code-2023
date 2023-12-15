use std::{fs, rc::Rc, cell::RefCell};

use itertools::Itertools;

fn tilt(table_view: &Vec<Rc<RefCell<char>>>, side_len: usize) -> &Vec<Rc<RefCell<char>>> {
    for j in 0..side_len {
        let mut i: usize = 0;
        let mut i0: usize = 0;
        let mut count: usize = 0;
        loop {
            while i < side_len && *table_view[side_len * i + j].borrow() != '#' {
                if *table_view[side_len * i + j].borrow() == 'O' { 
                    *table_view[side_len * i + j].borrow_mut() = '.';
                    count += 1; 
                }
                i += 1;
            }

            for k in 0..count 
                { *table_view[side_len * (i0 + k) + j].borrow_mut() = 'O'; }
            count = 0;
            i += 1;

            if i >= side_len { break; }
            i0 = i;
        }        
    }
    
    table_view
}

fn gen_view(table: &Vec<Rc<RefCell<char>>>, 
    view: &mut Vec<Rc<RefCell<char>>>,
    side_len: usize,
    map: fn(usize, usize, usize) -> usize
) 
{
    for i in 0..side_len {
        for j in 0..side_len {
            view[side_len * i + j] 
                = table[map(i, j, side_len)].clone();
        }
    }
}

fn cycle(table: &Vec<Rc<RefCell<char>>>, side_len: usize) {
    let mut view = table.clone();
    let dirs = [
        |i:usize, j:usize, l:usize| { i * l + j },
        |i, j, l| { j * l + i },
        |i, j, l| { l * (l - 1 - i) + j },
        |i, j, l| { l * j + (l - 1 - i) }
    ];
    {
        for map in dirs {
            gen_view(&table, &mut view, side_len, map);
            tilt(&view, side_len);
        }
    }

}

fn load(table: &Vec<Rc<RefCell<char>>>, side_len: usize) -> usize {
    let mut s1 = 0;
    for j in 0..side_len {
        for i in 0..side_len {
            s1 += if *table[i * side_len + j].borrow() == 'O' {side_len - i} else { 0 };
        }
    }
    s1
}

fn main() {
    let side_len = 100; 
    let raw_string = fs::read_to_string("./inputs/input14.txt")
        .expect("Unable to read file");

    let platform : Vec<Rc<RefCell<char>>> = raw_string.chars().filter(|c| *c != '\n')
        .map(|c| Rc::new(RefCell::new(c)))
        .collect_vec();
    {
        for _ in 0..500 {
            cycle(&platform, side_len);
        }
        let mut period: usize = 0;
        let start = platform.iter()
            .map(|c| Rc::new(RefCell::new(*c.borrow())))
            .collect_vec();
        loop {
            cycle(&start, side_len);
            period += 1;
            if platform == start {
                break;
            }
        }
        for _ in 0..((1000000000 - 500) % period) {
            cycle(&platform, side_len);
        }
        println!("{}", load(&platform, side_len));
    }
}