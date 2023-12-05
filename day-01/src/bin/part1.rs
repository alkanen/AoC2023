fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = String::new();
        let mut last_digit = String::new();

        // Find the first number in the line
        for c in line.chars() {
            if c.is_digit(10) {
                first_digit = c.to_string();
                break;
            }
        }

        // Find the last number in the line
        for c in line.chars().rev() {
            if c.is_digit(10) {
                last_digit = c.to_string();
                break;
            }
        }
        
        // Combine digits to a number and convert to integer
        let whole_number = first_digit + &last_digit;

        let number = whole_number.parse::<i32>().unwrap();

        println!("number: {}", number);

        sum += number;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142".to_string());
    }
}