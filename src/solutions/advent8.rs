use std::fs;
use crate::solutions::Solution;

pub struct Day8 {
    pub file: String
}


impl Solution for Day8 {
    fn solve(&self) {
        let text = fs::read_to_string(&self.file).expect("An error occured with file");
        let images = text.chars().collect::<Vec<_>>();
        let (width, height) = (25, 6);
        let layer = images.chunks(width * height)
                    .min_by(|c1, c2| c1.iter().filter(|&c| c == &'0').count().cmp(
                    &c2.iter().filter(|&c| c == &'0').count())
                );
        if let Some(min_layer) = layer {
            let res = min_layer.iter().filter(|&c| *c == '1').count() * 
                      min_layer.iter().filter(|&c| *c == '2').count();
            println!("{:?}", res);
        } else {
            println!("No layer found");
        }
        let mut password = vec!['2'; width * height];
        images.chunks(width * height).for_each(|layer| layer.iter().enumerate().for_each(|(i, col)|
            if password[i] == '2' {
                password[i] = *col;
            }));
        password.chunks(width).for_each(|line| println!("{}", line.iter()
        .map(|c| match c {
            '1' => "██",
            '0' => "  ",
            _ => "XX"
        }).collect::<String>()));
        // println!("{:?}", password);
    }
}
