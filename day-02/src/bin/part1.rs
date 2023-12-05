fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let (game_part, rounds_part) = line.split_once(": ").unwrap();
        let game = game_part.split_once(" ").unwrap().1.parse::<i32>().unwrap();
        //println!("Game {}", game);

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

        if !(max_red > 12 || max_green > 13 || max_blue > 14) {
        //    println!("  Invalid");
        //} else {
            //println!("  Valid");
            sum += game;
        }
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
");
        assert_eq!(result, "8".to_string());
    }
}
