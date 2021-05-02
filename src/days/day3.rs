pub fn day3(input: String) -> (usize, usize) {
    (part1(&"".to_string()), 0)
}

pub fn part1(input: &String) -> usize {
    let data = input
        .lines()
        .map(|l| l.trim().into())
        .collect::<Vec<String>>();
    let length = data.len();
    let column_length = data[0].len();
    let mut column = 0;
    let mut count = 0;
    for index in 1..length {
        column = (column + 3) % column_length;
        let c = data[index].chars().nth(column).unwrap();
        println!("Char found:{}", c);
        if c == '#' {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod part1 {
    #[test]
    fn example() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
            .to_string();
        let result = crate::days::day3::part1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn input() {
        let input = std::fs::read_to_string("input/3.txt").unwrap();
        let result = crate::days::day3::part1(&input);
        assert_eq!(result, 214);
    }
}
