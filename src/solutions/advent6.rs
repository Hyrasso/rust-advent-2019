use std::fs;
use std::collections::HashMap;
use crate::solutions::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn solve(&self) {
        let text = fs::read_to_string("inputs/6-1.txt").unwrap();
        let mut map = HashMap::new();
        // let mut dists = HashMap::new();
        text.lines()
            .map(|line| line.split(")").collect())
            .for_each(|line: Vec<_>| {
                map.insert(line[1], line[0]);
            });
        // println!("{:?}", map);
        let mut total = 0;
        for object in map.keys() {
            // eprintln!("D {:?}", object);
            let mut dist = 0;
            let mut node = object;
            while node != &"COM" {
                node = map.get(node).unwrap();
                dist += 1
            }
            total += dist;
        }
        println!("{:?}", total);
        println!("{:?}", map.get("YOU"));
        println!("{:?}", map.get("SAN"));
        let mut san_path = HashMap::new();
        let mut node = map.get("SAN").unwrap();
        let mut dist = 0;
        while node != &"COM" {
            let next_node = map.get(node).unwrap();
            san_path.insert(node, (next_node, dist));
            node = next_node;
            dist += 1;
        }
        // println!("{:?}", san_path);
        let mut node = &"YOU";
        dist = 0;
        while node != &"COM" {
            node = map.get(node).unwrap();
            if san_path.contains_key(node) {
                let (_, d) = san_path[node];
                dist += d;
                break;
            }
            dist += 1;
        }
        println!("{}", dist);

    }
}
