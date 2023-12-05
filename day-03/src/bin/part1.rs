fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn is_adjacent(symbol_positions: &Vec<(usize, usize, char)>, start_x: i32, end_x: i32, y: i32) -> bool {
    for symbol in symbol_positions.iter() {
        let (symbol_x, symbol_y, _) = symbol;
        let symbol_x = *symbol_x as i32;
        let symbol_y = *symbol_y as i32;

        if symbol_x == start_x - 1 && symbol_y == y {
            return true;
            //println!("  Left");
        }
        else if symbol_x == end_x + 1 && symbol_y == y {
            return true;
            //println!("  Right");
        }
        else if (symbol_x >= start_x -1 && symbol_x <= end_x +1) && symbol_y == y - 1 {
            return true;
            //println!("  Above ({} >= {} - 1 || {} <= {} + 1) && {} == {} - 1", symbol_x, start_x, symbol_x, end_x, symbol_y, pos_y);
        }
        else if (symbol_x >= start_x -1 && symbol_x <= end_x +1) && symbol_y == y + 1 {
            return true;
            //println!("  Below");
        }
    }
    return false;
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    // Create a list of the 2D coordinates of all symbols in the 
    // lines of the input string.
    let mut symbol_positions = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, symbol) in line.chars().enumerate() {
            // Don't include characters that are digits or dots.
            if symbol.is_digit(10) || symbol == '.' {
                continue;
            }
            symbol_positions.push((x, y, symbol));
        }
    }
    //println!("Symbol positions: {:?}", symbol_positions);

    // Go through each line of the input string, finding digits that are
    // grouped into numbers, and then figure out if they are adjacent to any
    // of the symbols stored in `symbol_positions`.
    for (y, line) in input.lines().enumerate() {
        let mut current_number = String::new();
        let mut start_position = -1;

        for (x, letter) in line.chars().enumerate() {
            if letter.is_digit(10) {
                current_number.push(letter);
                if start_position == -1 {
                    start_position = x as i32;
                }
            } else {
                if current_number.len() > 0 {
                    let end_position = x as i32 - 1;
                    let number = current_number.parse::<i32>().unwrap();

                    // Check if the number is adjacent to any of the symbols stored in
                    // `symbol_positions` by looking if it's either directly to the
                    // left, right, above, below, or diagonally adjacent to any of
                    // the symbols.
                    let adjacent = is_adjacent(&symbol_positions, start_position, end_position, y as i32);
                    if adjacent {
                        //println!("  Adjacent: {}", current_number);
                        sum += number;
                    }

                    current_number.clear();
                    start_position = -1;
                }
            }

            // Handle end of line.
            if x == line.len() - 1 {
                if current_number.len() > 0 {
                    let end_position = x as i32;
                    let number = current_number.parse::<i32>().unwrap();

                    // Check if the number is adjacent to any of the symbols
                    // stored in `symbol_positions`.
                    let adjacent = is_adjacent(&symbol_positions, start_position, end_position, y as i32);
                    if adjacent {
                        //println!("  Adjacent: {}", current_number);
                        sum += number;
                    }

                    current_number.clear();
                }
            }
        }
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
");
        assert_eq!(result, "4361".to_string());
    }
}