fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let data = parse_input(input);
    let empties = find_empty_rows_and_cols(&data);
    let expanded = expand_data(&data, &empties.0, &empties.1);
    let galaxies = find_galaxies(&expanded);
    let sum = sum_distances(&galaxies);
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

fn expand_data(input: &Vec<Vec<char>>, empty_rows: &Vec<usize>, empty_cols: &Vec<usize>) -> Vec<Vec<char>> {
    let mut output = input.clone();
    let blank_row = vec!['.'; input[0].len()];

    for row in empty_rows.iter().rev() {
        output.insert(*row, blank_row.clone());
    }

    for col in empty_cols.iter().rev() {
        for row in 0..output.len() {
            output[row].insert(*col, '.');
        }
    }

    return output;
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

fn sum_distances(input: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;

    for i in 0..input.len() {
        for j in i..input.len() {
            sum += (input[i].0 as i64 - input[j].0 as i64).abs() + (input[i].1 as i64 - input[j].1 as i64).abs();
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
    fn expand_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        let expanded = expand_data(&data, &empties.0, &empties.1);

        assert_eq!(expanded.len(), 12);
        assert_eq!(expanded[0], vec!['.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[1], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.']);
        assert_eq!(expanded[2], vec!['#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[3], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[4], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[5], vec!['.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.', '.']);
        assert_eq!(expanded[6], vec!['.', '#', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[7], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '#']);
        assert_eq!(expanded[8], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[9], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.']);
        assert_eq!(expanded[10], vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.', '.', '.']);
        assert_eq!(expanded[11], vec!['#', '.', '.', '.', '.', '#', '.', '.', '.', '.', '.', '.', '.']);
    }

    #[test]
    fn find_galaxies_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        let expanded = expand_data(&data, &empties.0, &empties.1);
        let galaxies = find_galaxies(&expanded);

        assert_eq!(galaxies.len(), 9);
        assert_eq!(galaxies[0], (0, 4));
        assert_eq!(galaxies[1], (1, 9));
        assert_eq!(galaxies[2], (2, 0));
        assert_eq!(galaxies[3], (5, 8));
        assert_eq!(galaxies[4], (6, 1));
        assert_eq!(galaxies[5], (7, 12));
        assert_eq!(galaxies[6], (10, 9));
        assert_eq!(galaxies[7], (11, 0));
        assert_eq!(galaxies[8], (11, 5));
    }

    #[test]
    fn sum_distances_test() {
        let data = parse_input(INPUT);
        let empties = find_empty_rows_and_cols(&data);
        let expanded = expand_data(&data, &empties.0, &empties.1);
        let galaxies = find_galaxies(&expanded);
        let sum = sum_distances(&galaxies);

        assert_eq!(sum, 374);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "374".to_string());
    }
}