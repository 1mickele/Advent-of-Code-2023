use std::fs;
use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec;

use itertools::Itertools;
use num::Integer;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Pulse {
    HIGH = 1,
    LOW = 0,
}

#[derive(Debug, Copy, Clone)]
pub enum ModuleEnum {
    Broadcast,
    FlipFlop,
    Conjunction
}

pub trait Module {
    fn compute(&mut self) -> (bool, Vec<Rc<RefCell<Pulse>>>, Pulse);
    fn broadcast(&self) -> (&Vec<Rc<RefCell<Pulse>>>, Pulse);
    fn typename(&self) -> &str;
    fn dump(&self) -> Vec<Rc<RefCell<Pulse>>>;
    fn output(&self) -> Pulse;
}

use core::fmt::Debug;
impl Debug for dyn Module {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Module{{{},{:?},->{:?}}}", self.typename(), self.dump(), self.output())
    }
}

// FlipFlop
#[derive(Debug)]
pub struct FlipFlop {
    input: Rc<RefCell<Pulse>>,
    broadcast_to: Vec<Rc<RefCell<Pulse>>>,
    output: Pulse,
}

impl FlipFlop {
    pub fn new(
        bt: Vec<Rc<RefCell<Pulse>>>,
        inp: Rc<RefCell<Pulse>>
    ) -> FlipFlop {
        FlipFlop { 
            input: inp,
            broadcast_to: bt,
            output: Pulse::LOW,
        }
    }  
}

impl Module for FlipFlop {
    fn compute(&mut self) -> (bool, Vec<Rc<RefCell<Pulse>>>, Pulse) {
        if *self.input.borrow() == Pulse::LOW {
            self.output = if self.output == Pulse::HIGH { Pulse::LOW }
                else { Pulse::HIGH };
            return (true, self.broadcast_to.clone(), self.output);
        }
        return (false, self.broadcast_to.clone(), self.output);
    }

    fn broadcast(&self) -> (&Vec<Rc<RefCell<Pulse>>>, Pulse) {
        (&self.broadcast_to, self.output)
    }

    fn typename(&self) -> &str {
        "FlipFlop"
    }

    fn dump(&self) -> Vec<Rc<RefCell<Pulse>>> {
        vec![self.input.clone()]
    }

    fn output(&self) -> Pulse {
        self.output
    }
}

// Conjunction
#[derive(Debug)]
pub struct Conjunction {
    inputs: Vec<Rc<RefCell<Pulse>>>,
    broadcast_to: Vec<Rc<RefCell<Pulse>>>,
    output: Pulse,
}

impl Conjunction {
    pub fn new(
        inp: Vec<Rc<RefCell<Pulse>>>,    
        bt: Vec<Rc<RefCell<Pulse>>>
    ) -> Conjunction {
        Conjunction { 
            inputs: inp,
            broadcast_to: bt,
            output: Pulse::LOW,
        }
    }
}

impl Module for Conjunction {
    fn compute(&mut self) -> (bool, Vec<Rc<RefCell<Pulse>>>, Pulse) {
        self.output = if self.inputs.iter().all(|x| *x.borrow() == Pulse::HIGH) { Pulse::LOW }
            else { Pulse::HIGH };
        return (true, self.broadcast_to.clone(), self.output);
    }

    fn broadcast(&self) -> (&Vec<Rc<RefCell<Pulse>>>, Pulse) {
        (&self.broadcast_to, self.output)
    }

    fn typename(&self) -> &str {
        "conjunction"
    }

    fn dump(&self) -> Vec<Rc<RefCell<Pulse>>> {
        self.inputs.clone()
    }

    fn output(&self) -> Pulse {
        self.output
    }
}

// Broadcast
#[derive(Debug)]
pub struct Broadcast {
    broadcast_to: Vec<Rc<RefCell<Pulse>>>,
    output: Pulse,
}

impl Broadcast {
    pub fn new(bt: Vec<Rc<RefCell<Pulse>>>) -> Broadcast {
        Broadcast { 
            broadcast_to: bt,
            output: Pulse::LOW,
        }
    }   
} 

impl Module for Broadcast {
    fn compute(&mut self) -> (bool, Vec<Rc<RefCell<Pulse>>>, Pulse) 
    { return (true, self.broadcast_to.clone(), self.output); }

    fn broadcast(&self) -> (&Vec<Rc<RefCell<Pulse>>>, Pulse) {
        (&self.broadcast_to, self.output)
    }

    fn typename(&self) -> &str {
        "broadcast"
    }

    fn dump(&self) -> Vec<Rc<RefCell<Pulse>>> {
        Vec::new()
    }

    fn output(&self) -> Pulse {
        self.output
    }
}

/*
    with this function we can inspect the graph and notice that each component
    we get by tracing back the nodes (indirectly) connected to sz, cm, xf, gc
    are pair-wise disjoint. Thus the overall network structure is

                      ┌─────────┐
                      │Broadcast│
          ┌───────────┴──┬──────┴────┬──────────┐
          │              │           │          │
┌─────────▼──────┐   ┌───▼──┐    ┌───▼──┐   ┌───▼──┐
│ Component1     │   │ Cpt2 │    │ Cpt3 │   │ Cpt4 │
│   (13 elem)    │   └──┬───┘    └──┬───┘   └───┬──┘
└─────────┬──────┘      │           │           │
          │             │           │           │
        ┌─▼──┐        ┌─▼──┐     ┌──▼─┐       ┌─▼──┐
        │ sz │        │ cm │     │ xf │       │ gc │
        └──┬─┘        └┬───┘     └─┬──┘       └──┬─┘
           │           │           │             │
           │         ┌─▼──┐◄───────┘             │
           └─────────► zr │                      │
                     └──┬─┘◄─────────────────────┘
                        │
                      ┌─▼┐
                      │rx│
                      └──┘
*/

#[allow(dead_code)]
fn traceback(inputs_from: &HashMap<String, (ModuleEnum, Vec<String>)>) {
    let sz = "sz".to_string();
    let cm = "cm".to_string();
    let xf = "xf".to_string();
    let gc = "gc".to_string();

    let mut components : Vec<Vec<&String>> = vec![
        vec![&sz], vec![&cm], vec![&xf], vec![&gc]
    ];

    for v in components.iter_mut() {
        let mut tbp : Vec<&String> = vec![v.first().unwrap()];
        while let Some(r) = tbp.pop() {
            let t = inputs_from.get(r).unwrap().1.iter()
                .filter(|s| !v.contains(s)).collect_vec();
            tbp.extend(t.iter());
            v.extend(t.iter());
        }
    }

    for (i,j) in vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
        println!("{}{}:{}", i, j, components.get(i).unwrap().iter()
            .filter(|x| components.get(j).unwrap().contains(x)).count());
    }

    dbg!(components);
}

fn parse() -> (
    HashMap<String, Box<dyn Module>>, 
    HashMap<String, (ModuleEnum, Vec<String>)>
) {
    let raw_string = fs::read_to_string("./inputs/input20.txt").expect("Unable to read file");
    let outputs_to: HashMap<String, (ModuleEnum, Vec<String>)> = HashMap::from_iter(
            raw_string.split('\n').map(|l| {
            let mut it = l.split(" -> ");
            let s = it.next().unwrap();
            let (tp, nm) = match s {
                _ if s.starts_with('&') => {
                    (ModuleEnum::Conjunction, s[1..].to_string())
                }
                _ if s.starts_with('%') => {
                    (ModuleEnum::FlipFlop, s[1..].to_string())
                }
                _ => {
                    (ModuleEnum::Broadcast, s.to_string())
                }
            };
            (nm, (tp, it.next().unwrap().split(", ")
                .map(|s| s.to_string()).collect_vec()))
        }));

    // let all_labels = outputs_to.values().flat_map(|(_, v)| v.iter())
    //    .chain(outputs_to.keys()).unique().collect_vec();

    let inputs_from: HashMap<String, (ModuleEnum, Vec<String>)> = HashMap::from_iter(
        outputs_to.iter().map(|(k, (t, _))| {
            (k.clone(), (*t, outputs_to.keys().filter(|k2| {
                outputs_to.get(*k2).unwrap().1.contains(k)
            }).map(|x| x.clone()).collect_vec()))
        })
    );

    // traceback(&inputs_from);

    // input
    let a : HashMap<String, Vec<Rc<RefCell<Pulse>>>> = HashMap::from_iter(
        inputs_from.iter().map(|(n, (t, v))| {
            (n.clone(), match t {
                ModuleEnum::Broadcast =>
                    Vec::from_iter((0..v.len()).map(|_| Rc::new(RefCell::new(Pulse::LOW)))),               
                ModuleEnum::Conjunction =>
                    Vec::from_iter((0..v.len()).map(|_| Rc::new(RefCell::new(Pulse::LOW)))),               
                ModuleEnum::FlipFlop => 
                    vec![Rc::new(RefCell::new(Pulse::LOW)); 1]
                    // Vec::from_iter((0..v.len()).map(|_| Rc::new(RefCell::new(Pulse::LOW)))),               
            })
        }));
    
    // output
    let b : HashMap<&String, Vec<Rc<RefCell<Pulse>>>> = HashMap::from_iter(
        outputs_to.iter().map(|(n, (_, v))| {
            (n, v.iter().map(|m| {
                match inputs_from.get(m) {
                    Some((ModuleEnum::FlipFlop, _)) => {
                        a.get(m).unwrap().first().unwrap().clone()
                    },
                    Some((ModuleEnum::Conjunction, v)) => {
                        let j = v.iter().position(|k| k == n).unwrap();
                        a.get(m).unwrap().get(j).unwrap().clone()
                    },
                    _ => { Rc::new(RefCell::new(Pulse::LOW)) } // unreachable
                }
            }).collect_vec())
        }));

    (HashMap::from_iter(
        inputs_from.iter().map(|(k, (t, _))| (k, t))
        .chain(iter::once((&"broadcaster".to_string(), &ModuleEnum::Broadcast)))
        .map(|(k, t)| 
            -> (String, Box<dyn Module>)
        {
            (k.clone(), match t {
                ModuleEnum::Broadcast => 
                    Box::new(Broadcast::new(b.get(&"broadcaster".to_string()).unwrap().clone())),
                ModuleEnum::Conjunction => 
                    Box::new(Conjunction::new(
                        a.get(k).unwrap().clone(), b.get(&k).unwrap().clone())),
                ModuleEnum::FlipFlop => 
                    Box::new(FlipFlop::new(
                         b.get(&k).unwrap().clone(), 
                            a.get(k).unwrap().first().unwrap().clone()))
            })
        })
    ), outputs_to) // fix .clone()?
}

fn main() {
    {
        let (mut table, outputs_to) = parse();
        let (mut l1, mut h1) = (0_usize, 0_usize);
        for _ in 0..1000 {
            let s = "broadcaster".to_string();
            let mut queue : VecDeque<(&String, Rc<RefCell<Pulse>>, Pulse)> 
                = VecDeque::from([(&s, Rc::new(RefCell::new(Pulse::LOW)), Pulse::LOW)]);
            while let Some((s, q, v)) = queue.pop_front() {
                *q.borrow_mut() = v;

                match v {
                    Pulse::HIGH => { h1 += 1; },
                    Pulse::LOW => { l1 += 1; }
                }
                
                if !table.contains_key(s) {
                    continue;
                }
                let (b, qs, w) = table.get_mut(s).unwrap().compute();
                if b {              
                    queue.extend(outputs_to.get(s).unwrap().1
                        .iter().zip(qs).map(|(n, b)| {
                            (n, b, w)
                        }));
                }
            }
        }
        // dbg!(&table, '\n');
        println!("{}", h1 * l1);
    }
    {
        let (mut table, outputs_to) = parse();
        let mut idxs : Vec<usize> = vec![0, 0, 0, 0];
        let mut i = 1;
        while idxs.iter().any(|x| *x == 0) {
            let s = "broadcaster".to_string();
            let mut queue : VecDeque<(&String, Rc<RefCell<Pulse>>, Pulse)> 
                = VecDeque::from([(&s, Rc::new(RefCell::new(Pulse::LOW)), Pulse::LOW)]);
            while let Some((s, q, v)) = queue.pop_front() {
                *q.borrow_mut() = v;
                if !table.contains_key(s) {
                    continue;
                }
                let (b, qs, w) = table.get_mut(s).unwrap().compute();
                if b {              
                    queue.extend(outputs_to.get(s).unwrap().1
                        .iter().zip(qs).map(|(n, b)| {
                            (n, b, w)
                        }));
                }

                if s == "zr" && w == Pulse::HIGH {
                    // sz, cm, xf, gc
                    if table.get("sz").unwrap().output() == Pulse::HIGH 
                        { idxs[0] = i; }
                    if table.get("cm").unwrap().output() == Pulse::HIGH 
                        { idxs[1] = i; }
                    if table.get("xf").unwrap().output() == Pulse::HIGH 
                        { idxs[2] = i; }
                    if table.get("gc").unwrap().output() == Pulse::HIGH 
                        { idxs[3] = i; }
                }
            }
            
            i += 1;
        }

        let s2 = idxs.into_iter().reduce(|a, b| a.lcm(&b)).unwrap();
        println!("{}", s2);
    }
}