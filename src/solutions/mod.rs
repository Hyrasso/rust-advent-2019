pub struct Day0 { pub solution: &'static str }

pub use self::advent1::Day1;
mod advent1;
pub use self::advent2::Day2;
mod advent2;
pub use self::advent3::Day3;
mod advent3;
pub use self::advent4::Day4;
mod advent4;
pub use self::advent5::Day5;
mod advent5;
pub use self::advent6::Day6;
mod advent6;
pub use self::advent7::Day7;
mod advent7;
pub use self::advent8::Day8;
mod advent8;

pub trait Solution {
    fn solve(&self);
}

impl Solution for Day0 {
    fn solve(&self) {
        println!("{}", self.solution)
    }
}
