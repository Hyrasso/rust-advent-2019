use std::fs;
use crate::solutions::Solution;
use std::collections::HashSet;
use std::cmp::Ordering;

pub struct Day10 {
    pub file: String
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        (self.y as f64).atan2(self.x as f64).partial_cmp(
            &(other.y as f64).atan2(other.x as f64)
        ).unwrap()
    }
}

impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    fn sym(&self) -> Point {
        Point {
            x: self.y,
            y: self.x
        }
    }

    fn is_inside(&self, limit: &Point) -> bool {
        self.x >= 0 && self.x <= limit.x && self.y >= 0 && self.y <= limit.y
    }

    fn rotate(&self, quad: i32) -> Point {
        match quad {
            0 => self.clone(),
            1 => Point {
                x: -self.y,
                y: self.x
            },
            2 => Point {
                x: -self.x,
                y: -self.y
            },
            3 => Point {
                x: self.y,
                y: -self.x
            },
            _ => panic!("Unknown quad {:?}", quad)
        }
    }
}

/** https://en.wikipedia.org/wiki/Farey_sequence
*/
fn farey(n: i32) -> Vec<Point> {
    if n < 1 {
        return vec![];
    }

    let (mut a, mut b, mut c, mut d) = (0, 1, 1, n);
    let mut res = vec![];
    res.push(Point {x: a, y: b});
    while c <= n {
        let k = (n + b) / d;
        let abcd = (c, d, k * c - a, k * d - b);
        a = abcd.0;
        b = abcd.1;
        c = abcd.2;
        d = abcd.3;
        res.push(Point {x: a, y: b});
    }
    res
}

impl Solution for Day10 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("Error while opening input");
        let (width, height) = (text.find('\n').unwrap() as i32 - 1, text.lines().count() as i32);
        let limit = Point {x: width, y: height};
        let mut asteroids = HashSet::new();
        for (y, line) in text.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    asteroids.insert(Point {
                        x: x as i32,
                        y: y as i32
                    });
                }
            }
        }
        println!("Size {} {}", width, height);
        // println!("Map {:?}", asteroids);
        // best 11, 19
        let mut destroyed = HashSet::new();
        // for start in vec![Point {x: 11, y: 19}] {
        for start in vec![Point {x: 11, y: 13}] {
            // clockwise, symetric
            while asteroids.len() > 1 {
                for quadrant in vec![3, 0, 1, 2] {
                    eprintln!("Quadrant {}", quadrant);
                    let skip_start = if destroyed.len() == 0 { 0 } else { 1 };
                    for point in farey(height).into_iter()
                                .chain(farey(width).into_iter().skip(skip_start).rev().skip(1).map(|p| p.sym()))
                                .map(|p| p.rotate(quadrant))
                                .rev() {
                        let mut ray_vec = point;
                        // eprintln!("Dir {:?}", point);
                        while start.add(&ray_vec).is_inside(&limit) {
                            if asteroids.contains(&start.add(&ray_vec)) {
                                let asteroid = start.add(&ray_vec);
                                eprintln!("Destroyed {:?} {} {:?}", point, destroyed.len() + 1, asteroid);
                                asteroids.remove(&asteroid);
                                assert_eq!(destroyed.insert(asteroid), true);
                                break;
                            }
                            ray_vec = ray_vec.add(&point);
                        }
                    }
                }
                println!("{:?}", destroyed.len());
            }
        }
    }
}