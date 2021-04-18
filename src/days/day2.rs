use regex::Regex;

pub fn day2(input: String) -> (usize, usize) {
    (part1(input), 0)
}

fn part1(input: String) -> usize {
    let reg = Regex::new(r"(\d+)-(\d+) ([a-z]): (.+)").unwrap();
    let input = input.trim();
    input
        .split("\n")
        .map(str::trim)
        .map(|l| {
            let groups = reg.captures(l).unwrap();
            let start = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let stop = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();

            let letter = groups.get(3).unwrap().as_str();
            let password = groups.get(4).unwrap().as_str();

            let occurences = password.matches(letter).count();
            occurences >= start && occurences <= stop
        })
        .filter(|p| *p == true)
        .count()
}
#[cfg(test)]
mod day2 {
    mod part1 {
        use crate::days::day2::day2;

        #[test]
        fn test_part1_example() {
            let input = "1-3 a: abcde
      1-3 b: cdefg
      2-9 c: ccccccccc"
                .to_string();

            let result = day2(input);
            assert_eq!(result.0, 2)
        }

        #[test]
        fn test_part1_input() {
            let input = std::fs::read_to_string("input/2.txt").unwrap();
            let result = day2(input);
            assert_eq!(result.0, 560);
        }
    }
}
