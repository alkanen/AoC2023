fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn adjacent_numbers(number_positions: &Vec<(i32, i32, i32, i32)>, x: i32, y: i32) -> Vec<i32> {
    // Check if the given position is adjacent to exactly two numbers.
    let mut numbers = Vec::new();

    for number in number_positions.iter() {
        let (start_x, end_x, pos_y, number) = number;
        let start_x = *start_x;
        let end_x = *end_x;
        let pos_y = *pos_y;

        // To the left
        if x == start_x - 1 && y == pos_y {
            numbers.push(*number);
        }
        // To the right
        else if x == end_x + 1 && y == pos_y {
            numbers.push(*number);
        }
        // Above
        else if (x >= start_x -1 && x <= end_x +1) && y == pos_y - 1 {
            numbers.push(*number);
        }
        // Below
        else if (x >= start_x -1 && x <= end_x +1) && y == pos_y + 1 {
            numbers.push(*number);
        }
    }

    return numbers;
}

fn part2(input: &str) -> String {
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

    // Go through each line of the input string, finding digits that are
    // grouped into numbers, and then figure out if they are adjacent to any
    // of the symbols stored in `symbol_positions`.
    let mut number_positions = Vec::new();
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

                    number_positions.push((start_position, end_position, y as i32, number));

                    current_number.clear();
                    start_position = -1;
                }
            }

            // Handle end of line.
            if x == line.len() - 1 {
                if current_number.len() > 0 {
                    let end_position = x as i32;
                    let number = current_number.parse::<i32>().unwrap();

                    number_positions.push((start_position, end_position, y as i32, number));

                    current_number.clear();
                }
            }
        }
    }

    // Go through symbols and see if any of them are connected to exactly two numbers.
    for symbol in symbol_positions {
        let (symbol_x, symbol_y, symbol_char) = symbol;
        if symbol_char == '*' {
           let adjacent_numbers = adjacent_numbers(&number_positions, symbol_x as i32, symbol_y as i32);

            // If the symbol is adjacent to exactly two numbers, add its value to the sum.
            if adjacent_numbers.len() == 2 {
                let ratio = adjacent_numbers[0] * adjacent_numbers[1];
                sum += ratio;
            }
        }
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let result = part2("467..114..
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
        assert_eq!(result, "467835".to_string());
    }
}