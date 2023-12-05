fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (_game_part, rounds_part) = line.split_once(": ").unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        //println!("Rounds: {}", rounds_part);
        let rounds = rounds_part.split("; ");
        for round in rounds {
            //println!("  Round: {}", round);
            for cube in round.split(", ") {
                let (count, color) = cube.split_once(" ").unwrap();
                let count = count.parse::<i32>().unwrap();
                //println!("    {} {}", count, color);

                if color == "red" {
                    if count > max_red {
                        max_red = count;
                    }
                } else if color == "green" {
                    if count > max_green {
                        max_green = count;
                    }
                } else if color == "blue" {
                    if count > max_blue {
                        max_blue = count;
                    }
                }
            }
        }

        let power = max_red * max_green * max_blue;
        //println!("  red: {}, green: {}, blue: {}", max_red, max_green, max_blue);
        //println!("  Power in game {}: {}", game, power);

        sum += power;
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
");
        assert_eq!(result, "2286".to_string());
    }
}
