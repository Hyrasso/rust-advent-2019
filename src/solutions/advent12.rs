// #[macro_use] extern crate itertools;
use std::fs;
use crate::solutions::Solution;
use std::collections::HashMap;

// Rosetta code
use std::cmp::{max, min};
 
fn gcd(a: i128, b: i128) -> i128 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}

fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}

pub struct Day12 {
    pub file: String
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32
}

impl Moon {
    fn new(pos: Vec<i32>) -> Self {
        assert_eq!(pos.len(), 3);
        Moon {
            x: pos[0],
            y: pos[1],
            z: pos[2],
            vx: 0,
            vy: 0,
            vz: 0
        }
    }

    fn energy(&self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) *
        (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }

    fn apply_gravity(&mut self, others: &[Moon]) {
        for moon in others {
            self.vx += (moon.x - self.x).signum();
            self.vy += (moon.y - self.y).signum();
            self.vz += (moon.z - self.z).signum();
        }
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }
}

impl Solution for Day12 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("Error while opening input");
        let mut moons: Vec<Moon> = text.lines().map(|line| line.split(",")
                                            .map(|pos| pos.parse::<i32>().unwrap())
                                            .collect::<Vec<_>>())
                                .map(|pos| Moon::new(pos))
                                .collect();
        // println!("{:?}", moons);
        let start = moons.clone();
        let mut cycles: HashMap<usize, usize> = HashMap::new();
        for i in 0..usize::max_value() {
            // println!("After step {}", i);
            // moons.iter().for_each(|m| println!("{:?}", m));
            
            let moons_before = moons.clone();
            for j in 0..moons.len() {
                moons[j].apply_gravity(&moons_before);
            }
            for j in 0..moons.len() {
                moons[j].apply_velocity();
            }
            for u in 0..3 {
                match u {
                    0 => if moons.iter().enumerate()
                                .all(|(j, m)| m.x == start[j].x && m.vx == start[j].vx)
                             && cycles.get(&u).is_none() {
                        cycles.insert(0, i + 1);
                    },
                    1 => if moons.iter().enumerate()
                                .all(|(j, m)| m.y == start[j].y && m.vy == start[j].vy)
                             && cycles.get(&u).is_none() {
                        cycles.insert(1, i + 1);
                    },
                    2 => if moons.iter().enumerate()
                                .all(|(j, m)| m.z == start[j].z && m.vz == start[j].vz)
                             && cycles.get(&u).is_none() {
                        cycles.insert(2, i + 1);
                    },
                    _ => ()
                }
            }
            if cycles.len() >= 3 {
                break;
            }

        }
        // println!("{:?}", moons.iter().map(|m| {
        //     println!("{:?}", m);
        //     m.energy()
        // }).sum::<i32>());
        println!("{:?}", cycles);
        println!("{:?}", cycles.iter().fold(1, |acc, (_k, v)| lcm(acc, *v as i128)));

    }
}