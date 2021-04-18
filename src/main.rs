mod days;
use std::{collections::HashMap, env};
fn main() {
    let mut days = HashMap::<String, &dyn Fn(String) -> (usize, usize)>::new();

    days.insert("1".to_string(), &days::day1::day1);
    days.insert("2".to_string(), &days::day2::day2);
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let day = &args[1];
        let input = std::fs::read_to_string("input/1.txt").unwrap();
        let day_function = days.get(day).unwrap();
        let (part1, part2) = day_function(input);
        println!("Part1: {}, part2: {}", part1, part2);
    }
}
