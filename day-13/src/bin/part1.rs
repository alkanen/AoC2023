fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let chunks = parse(input);

    let result = calculate_score(&chunks);

    return result.to_string();
}

fn parse(input: &str) -> Vec<(Vec<i64>, Vec<i64>)> {
    // The input is a list of lines. The lines are grouped into larger chunks
    // separated by blank lines. Each chunk has multiple lines with hash signs
    // and dots. The hash signs represent the bit 1, and the dots represent the
    // bit 0. The chunks are separated by blank lines.

    // Parse each chunk and convert into it's binary representation and store in
    // an i64.
    let mut chunks = Vec::new();

    for chunk in input.split("\n\n") {
        let mut grid: Vec<Vec<i64>> = Vec::new();
        for line in chunk.lines() {
            let mut row: Vec<i64> = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => row.push(1),
                    '.' => row.push(0),
                    _ => panic!("unexpected character"),
                }
            }
            grid.push(row);
        }

        // Convert the grid rows into a binary representation.
        let mut row_binary: Vec<i64> = Vec::new();
        for row in grid.iter() {
            let mut value = 0;
            for bit in row {
                value <<= 1;
                value |= bit;
            }
            row_binary.push(value as i64);
        }

        let mut col_binary: Vec<i64> = Vec::new();
        for col in 0..grid[0].len() {
            let mut value = 0;
            for row in 0..grid.len() {
                value <<= 1;
                value |= grid[row][col];
            }
            col_binary.push(value as i64);
        }

        // Convert the binary representation into a decimal representation.
        /*
        let mut decimal = Vec::new();
        for value in binary {
            decimal.push(value as i64);
        }
        */

        // Add the chunk to the list of chunks.
        chunks.push((row_binary, col_binary));
    }

    return chunks
}

fn find_symmetry_line(chunk: &(Vec<i64>, Vec<i64>)) -> (Option<usize>, Option<usize>) {
    let (rows, cols) = chunk;

    let mut row_start = None;
    let mut col_start = None;

    for i in 0..rows.len()-1 {
        if rows[i] == rows[i+1] {
            // Backtrack here and make sure both sides are symmetrical
            // all the way to the nearest edge.
            let mut left = i;
            let mut right = i+1;
            while left > 0 && right < rows.len()-1 {
                if rows[left-1] != rows[right+1] {
                    break;
                }
                left -= 1;
                right += 1;
            }

            if left == 0 || right == rows.len()-1 {
                row_start = Some(i);
                break;
            }
        }
    }

    for j in 0..cols.len()-1 {
        if cols[j] == cols[j+1] {
            // Backtrack here and make sure both sides are symmetrical
            // all the way to the nearest edge.
            let mut top = j;
            let mut bottom = j+1;
            while top > 0 && bottom < cols.len()-1 {
                if cols[top-1] != cols[bottom+1] {
                    break;
                }
                top -= 1;
                bottom += 1;
            }

            if top == 0 || bottom == cols.len()-1 {
                col_start = Some(j);
                break;
            }
        }
    }


    return (row_start, col_start)
}

fn calculate_score(chunks: &Vec<(Vec<i64>, Vec<i64>)>) -> i64 {
    let mut score = 0;

    for chunk in chunks {
        let (row_line, col_line) = find_symmetry_line(chunk);

        if row_line.is_some() {
            let row_line = row_line.unwrap();
            score += (row_line + 1) * 100;
        }

        if col_line.is_some() {
            let col_line = col_line.unwrap();
            score += col_line + 1;
        }
    }

    return score as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_parse_single() {
        let input = "#.#.#.#."; // 0b10101010 -> 170
        let chunks = parse(input);
        assert_eq!(chunks.len(), 1);

        let (rows, cols) = &chunks[0];
        assert_eq!(rows.len(), 1);
        assert_eq!(cols.len(), 8);

        // There's a single row value
        assert_eq!(rows[0], 170);

        // There are a bunch of col values
        assert_eq!(cols[0], 1);
        assert_eq!(cols[1], 0);
        assert_eq!(cols[2], 1);
        assert_eq!(cols[3], 0);
        assert_eq!(cols[4], 1);
        assert_eq!(cols[5], 0);
        assert_eq!(cols[6], 1);
        assert_eq!(cols[7], 0);
    }

    #[test]
    fn test_parse_two_chunks() {
        let chunks = parse(INPUT);
        assert_eq!(chunks.len(), 2);

        let num_row_values_in_chunk = vec![7, 7];
        let num_col_values_in_chunk = vec![9, 9];

        for (i, (rows, cols)) in chunks.iter().enumerate() {
            println!("chunk {}", i);
            println!("  rows: {:?}", rows);
            println!("  cols: {:?}", cols);
            assert_eq!(rows.len(), num_row_values_in_chunk[i]);
            assert_eq!(cols.len(), num_col_values_in_chunk[i]);
        }
    }

    #[test]
    fn test_find_symmetry() {
        let chunks = parse(INPUT);
        assert_eq!(chunks.len(), 2);

        let (row_start, col_start) = find_symmetry_line(&chunks[0]);
        assert_eq!(row_start, None);
        assert_eq!(col_start, Some(4));

        let (row_start, col_start) = find_symmetry_line(&chunks[1]);
        assert_eq!(row_start, Some(3));
        assert_eq!(col_start, None);
    }

    #[test]
    fn test_calculate_score() {
        let chunks = parse(INPUT);
        assert_eq!(chunks.len(), 2);

        let result = calculate_score(&chunks);
        assert_eq!(result, 405);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "405".to_string());
    }
}