use std::cmp;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let data = parse_input(input);
    let empties = find_empty_rows_and_cols(&data);
    let galaxies = find_galaxies(&data);
    let sum = sum_distances(&galaxies, &empties.0, &empties.1);
    return sum.to_string();
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|line| line.chars().collect()).collect();
}

fn find_empty_rows_and_cols(input: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    // Rows
    for (row_index, row) in input.iter().enumerate() {
        let mut is_empty = true;
        for char in row {
            if char == &'#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_rows.push(row_index);
        }
    }

    // Cols
    for col in 0..input[0].len() {
        let mut is_empty = true;
        for row in 0..input.len() {
            if input[row][col] == '#' {
                is_empty = false;
                break;
            }
        }

        if is_empty {
            empty_cols.push(col);
        }
    }

    return (empty_rows, empty_cols);
}

fn expand_axes(input: &Vec<Vec<char>>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>, dist: usize) -> (Vec<usize>, Vec<usize>) {
    // Calculate two new vectors with the x and y axis where each entry has the distance 
    // to the next coordinate point.
    let mut x_axis: Vec<usize> = Vec::new();
    let mut y_axis: Vec<usize> = Vec::new();
    for y in 0..input.len() {
        let mut is_empty = false;
        for empty in empty_rows {
            if *empty == y {
                is_empty = true;
                break;
            }
        }

        if is_empty {
            y_axis.push(dist - 1);
        } else {
            y_axis.push(1);
        }
    }

    return (x_axis, y_axis);
}

fn find_galaxies(input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for row in 0..input.len() {
        for col in 0..input[0].len() {
            if input[row][col] == '#' {
                galaxies.push((row, col));
            }
        }
    }

    return galaxies;
}

fn sum_distances(input: &Vec<(usize, usize)>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> usize {
    let mut sum = 0;
    let empty_dist = 1000000-1;

    for i in 0..input.len() {
        let ix = input[i].1;
        let iy = input[i].0;
        for j in i..input.len() {
            let mut empty = 0;
            let jx = input[j].1;
            let jy = input[j].0;

            let lowx = cmp::min(ix, jx);
            let highx = cmp::max(ix, jx);
            let lowy = cmp::min(iy, jy);
            let highy = cmp::max(iy, jy);

            for row in empty_rows {
                if *row > lowy && *row <= highy {
                    println!("row: {} is within {} and {}", row, lowy, highy);
                    empty += 1;
                }
            }
            if empty > 0 {
                println!("  empty: {}", empty);
            }

            for col in empty_cols {
                if *col > lowx && *col <= highx {
                    empty += 1;
                }
            }

            sum += (input[i].0 as i64 - input[j].0 as i64).abs() + (input[i].1 as i64 - input[j].1 as i64).abs() + empty_dist * empty;
        }
    }

    return sum as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn empty_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        assert_eq!(empties, (vec![3, 7], vec![2, 5, 8]));
    }

    #[test]
    fn find_galaxies_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        let galaxies = find_galaxies(&data);

        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0], (0, 3));
        assert_eq!(galaxies[1], (1, 7));
        assert_eq!(galaxies[2], (2, 0));
        assert_eq!(galaxies[3], (4, 6));
        assert_eq!(galaxies[4], (5, 1));
        assert_eq!(galaxies[5], (6, 9));
        assert_eq!(galaxies[6], (8, 7));
        assert_eq!(galaxies[7], (9, 0));
        assert_eq!(galaxies[8], (9, 4));
    }

    #[test]
    fn sum_distances_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        let galaxies = find_galaxies(&data);
        let sum = sum_distances(&galaxies, &empties.0, &empties.1);

        assert_eq!(sum, 374);
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "374".to_string());
    }
}