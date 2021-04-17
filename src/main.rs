use itertools::Itertools;

fn main() {}

#[cfg(test)]
mod tests {
    use std::str;

    use itertools::Itertools;

    #[test]
    fn test_part1_example() {
        let input = "1721
        979
        366
        299
        675
        1456
        ";

        let result = solve(input);
        assert_eq!(result, 514579);
    }

    fn solve(input: &str) -> usize {
        let numbers: Vec<usize> = input
            .split("\n")
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect::<Vec<usize>>();

        println!("{:?}", numbers);
        let result = numbers
            .into_iter()
            .combinations(2)
            .find(|x| x.into_iter().sum::<usize>() == 2020)
            .unwrap();

        result[0] * result[1]
    }
}
