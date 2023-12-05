fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (_card_part, rounds_part) = line.split_once(": ").unwrap();
        //let card = card_part.split_once(" ").unwrap().1.parse::<i32>().unwrap();

        let (winning_numbers_part, own_numbers_part) = rounds_part.split_once(" | ").unwrap();
        let winning_numbers = winning_numbers_part.split_whitespace().collect::<Vec<&str>>();
        let own_numbers = own_numbers_part.split_whitespace().collect::<Vec<&str>>();

        //println!("Card {}:", card);
        //println!("  Winning numbers:");
        let mut score = 0;
        for winning_number in winning_numbers.iter() {
            //print!("    {}", winning_number);
            if own_numbers.contains(winning_number) {
                //print!(" (own)");
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
            //println!("")
        }
        //println!("  Score: {}", score);
        sum += score;
    // println!("  Own numbers: {}", own_numbers);
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
");
        assert_eq!(result, "13".to_string());
    }
}