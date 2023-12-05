fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn convert_to_number(line: &str, i: usize) -> i32 {

    if line[i..].starts_with("one") {
        return 1;
    }
    if line[i..].starts_with("two") {
        return 2;
    }
    if line[i..].starts_with("three") {
        return 3;
    }
    if line[i..].starts_with("four") {
        return 4;
    }
    if line[i..].starts_with("five") {
        return 5;
    }
    if line[i..].starts_with("six") {
        return 6;
    }
    if line[i..].starts_with("seven") {
        return 7;
    }
    if line[i..].starts_with("eight") {
        return 8;
    }
    if line[i..].starts_with("nine") {
        return 9;
    }
    
    return -1;
}

fn part2(input: &str) -> String {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut first_digit: i32 = 0;
        let mut last_digit: i32 = 0;

        // Find the first number in the line
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                // Interpret digit as integer
                first_digit = c.to_digit(10).unwrap() as i32;
                break;
            }

            let tmp = convert_to_number(line, i);
            if tmp >= 0 {
                first_digit = tmp;
                break;
            }
        }

        // Find the last number in the line
        for (i, c) in line.chars().rev().enumerate() {
            if c.is_digit(10) {
                // Interpret digit as integer
                last_digit = c.to_digit(10).unwrap() as i32;
                break;
            }

            // println!("First character: {}", c.to_string());
            let tmp = convert_to_number(line, line.len() - i - 1);
            if tmp >= 0 {
                last_digit = tmp;
                break;
            }
        }
        
        // Combine digits to a number and convert to integer
        let number = first_digit * 10 + last_digit;

        sum += number;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281".to_string());
    }
}