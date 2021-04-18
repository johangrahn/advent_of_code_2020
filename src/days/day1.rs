use itertools::Itertools;

pub fn day1(input: String) -> (usize, usize) {
    let input: Vec<usize> = input
        .split("\n")
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let part1 = solve(input.clone());
    let part2 = solve_part2(input);

    (part1, part2)
}

fn solve(input: Vec<usize>) -> usize {
    let result = input
        .into_iter()
        .combinations(2)
        .find(|x| x.into_iter().sum::<usize>() == 2020)
        .unwrap();

    result[0] * result[1]
}

fn solve_part2(input: Vec<usize>) -> usize {
    let result = input
        .into_iter()
        .combinations(3)
        .find(|v| v.iter().sum::<usize>() == 2020)
        .unwrap();

    result[0] * result[1] * result[2]
}

#[cfg(test)]
mod tests {
    mod part1 {
        use crate::days;

        #[test]
        fn test_part1_example() {
            let input = "1721
       979
      366
      299
      675
      1456
      ";
            let result = days::day1::day1(input.to_string());
            assert_eq!(result.0, 514579);
        }

        #[test]
        fn test_part1_input() {
            let input = std::fs::read_to_string("input/1.txt").unwrap();
            let result = days::day1::day1(input.to_string());
            assert_eq!(result.0, 436404);
        }
    }

    mod part2 {
        #[test]
        fn test_part2_example() {
            let input = "1721
          979
         366
         299
         675
         1456
        ";
            let result = crate::days::day1::day1(input.to_string());
            assert_eq!(result.1, 241861950);
        }

        #[test]
        fn test_part2_input() {
            let input = std::fs::read_to_string("input/1.txt").unwrap();

            let result = crate::days::day1::day1(input.to_string());
            assert_eq!(result.1, 274879808);
        }
    }
}
