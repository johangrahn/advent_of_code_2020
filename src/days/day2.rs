use regex::Regex;

pub fn day2(input: String) -> (usize, usize) {
    let reg = Regex::new(r"(\d+)-(\d+) ([a-z]): (.+)").unwrap();
    let input = input.trim();
    let input = input.split("\n").map(str::trim).collect::<Vec<&str>>();

    let p1 = input
        .iter()
        .map(|l| p1(l, &reg))
        .filter(|p| *p == true)
        .count();
    let p2 = input
        .iter()
        .map(|l| p2(l, &reg))
        .filter(|p| *p == true)
        .count();

    (p1, p2)
}

fn p1(l: &str, reg: &Regex) -> bool {
    let groups = reg.captures(l).unwrap();
    let start = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let stop = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();

    let letter = groups.get(3).unwrap().as_str();
    let password = groups.get(4).unwrap().as_str();

    let occurences = password.matches(letter).count();
    occurences >= start && occurences <= stop
}

fn p2(l: &str, reg: &Regex) -> bool {
    let groups = reg.captures(l).unwrap();
    let first = groups.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let second = groups.get(2).unwrap().as_str().parse::<usize>().unwrap();

    let letter = groups.get(3).unwrap().as_str().chars().nth(0).unwrap();
    let password = groups.get(4).unwrap().as_str().to_string();

    if let Some(f) = password.chars().nth(first - 1) {
        if let Some(s) = password.chars().nth(second - 1) {
            return (f == letter) ^ (s == letter);
        }
    }
    false
}

#[cfg(test)]
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

mod part2 {
    use crate::days::day2::day2;

    #[test]
    fn test_part2_example() {
        let input = "1-3 a: abcde
      1-3 b: cdefg
      2-9 c: ccccccccc"
            .to_string();

        let result = day2(input);
        assert_eq!(result.1, 1)
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("input/2.txt").unwrap();
        let result = day2(input);
        assert_eq!(result.1, 303);
    }
}
