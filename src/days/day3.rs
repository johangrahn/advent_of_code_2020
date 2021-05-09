pub fn day3(input: String) -> (usize, usize) {
    let data = input
        .lines()
        .map(|l| l.trim().into())
        .collect::<Vec<String>>();

    (part1(&data), part2(&data))
}

pub fn part1(data: &Vec<String>) -> usize {
    let column_length = data[0].len();

    calc_trees(&data, column_length, 1, 3)
}

pub fn part2(data: &Vec<String>) -> usize {
    let column_length = data[0].len();
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes
        .iter()
        .map(|slope| calc_trees(data, column_length, slope.0, slope.1))
        .product::<usize>()
}

fn calc_trees(
    data: &Vec<String>,
    column_length: usize,
    row_jump: usize,
    column_jump: usize,
) -> usize {
    let mut row = row_jump;
    let mut column = column_jump;
    let mut count = 0;

    // for row in row_jump..length {
    //     column = (column + column_jump) % column_length;
    //     let c = data[row].chars().nth(column).unwrap();
    //     if c == '#' {
    //         count += 1;
    //     }
    // }

    println!("Calculating: {},{}", row, column);
    loop {
        if let Some(line) = data.get(row) {
            let c = line.chars().nth(column).unwrap();
            if c == '#' {
                count += 1;
            }

            column = (column + column_jump) % column_length;
            row += row_jump
        } else {
            break;
        }
    }

    println!("Found trees: {}", count);
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
        let result = crate::days::day3::day3(input);
        assert_eq!(result.0, 7);
    }

    #[test]
    fn input() {
        let input = std::fs::read_to_string("input/3.txt").unwrap();
        let result = crate::days::day3::day3(input);
        assert_eq!(result.0, 214);
    }
}

mod part2 {
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
        let result = crate::days::day3::day3(input);
        assert_eq!(result.1, 336);
    }

    #[test]
    fn input() {
        let input = std::fs::read_to_string("input/3.txt").unwrap();
        let result = crate::days::day3::day3(input);
        assert_eq!(result.1, 8336352024);
    }
}
