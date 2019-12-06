use std::fs;
use crate::solutions::Solution;

pub struct Day3 {}

#[derive(Debug)]
struct Segment {
    start: (i32, i32),
    end: (i32, i32)
}

impl Segment {
    fn intersect(&self, other: &Segment) -> Option<(i32, i32)> {
        if self.is_vert() == other.is_vert() {
            return None;
        }
        let x = self.start.0;
        let y = self.start.1;
        if self.is_vert() {
            if other.start.0 < x && x < other.end.0 && 
                other.start.1 > self.start.1 && other.start.1 < self.end.1 {
                // println!("{:?} + {:?} : {:?}", self, other, (x, other.start.1));
                return Some((x, other.start.1));
            } else {
                return None;
            }
        } else {
            if other.start.1 < y && y < other.end.1 && 
                other.start.0 > self.start.0 && other.start.0 < self.end.0 {
                // println!("{:?} + {:?} : {:?}", self, other, (other.start.0, y));
                return Some((other.start.0, y));
            } else {
                return None;
            }
        }
    }

    fn is_vert(&self) -> bool {
        self.start.0 == self.end.0
    }
}
impl Solution for Day3 {
    fn solve(&self) {
        let text = fs::read_to_string("inputs/3-1.txt").unwrap();
        // let text = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let mut lines = text.lines();
        let wire1 = lines.next().unwrap();
        let wire2 = lines.next().unwrap();

        let mut start = (0, 0);
        let mut wire1_segments = vec!();
        for elem in wire1.split(",") {
            let (dir, dist) = elem.split_at(1);
            let dist = dist.parse::<i32>().unwrap();
            start = match dir {
                "U" => {
                    let end = (start.0, start.1 + dist);
                    wire1_segments.push(Segment{start: start, end:end});
                    end
                },
                "D" => {
                    let end = (start.0, start.1 - dist);
                    wire1_segments.push(Segment{start: end, end:start});
                    end
                },
                "L" => {
                    let end = (start.0 - dist, start.1);
                    wire1_segments.push(Segment{start: end, end:start});
                    end
                },
                "R" => {
                    let end = (start.0 + dist, start.1);
                    wire1_segments.push(Segment{start: start, end: end});
                    end
                },
                _ => {println!("Unknown dir {}", dir); start}
            };
        }
        let mut start = (0, 0);
        let mut intersecs = vec!();
        for elem in wire2.split(",") {
            let (dir, dist) = elem.split_at(1);
            let dist = dist.parse::<i32>().unwrap();
            start = match dir {
                "U" => {
                    let end = (start.0, start.1 + dist);
                    let segment = Segment{start: start, end:end};
                    intersecs.extend(wire1_segments.iter().map(|s| segment.intersect(&s)).filter_map(|point| point));
                    end
                },
                "D" => {
                    let end = (start.0, start.1 - dist);
                    let segment = Segment{start: end, end: start};
                    intersecs.extend(wire1_segments.iter().map(|s| segment.intersect(&s)).filter_map(|point| point));
                    end
                },
                "L" => {
                    let end = (start.0 - dist, start.1);
                    let segment = Segment{start: end, end: start};
                    intersecs.extend(wire1_segments.iter().map(|s| segment.intersect(&s)).filter_map(|point| point));
                    end
                },
                "R" => {
                    let end = (start.0 + dist, start.1);
                    let segment = Segment{start: start, end:end};
                    intersecs.extend(wire1_segments.iter().map(|s| segment.intersect(&s)).filter_map(|point| point));
                    end
                },
                _ => {println!("Unknown dir {}", dir); start}
            };
        }
        println!("{:?}", intersecs.iter().map(|(x, y)| x.abs() + y.abs()).min());
    }
}