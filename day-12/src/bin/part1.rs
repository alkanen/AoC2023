/*
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending},
    combinator::map_res,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
*/

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let springs_groups = parse_input(input);

    return calculate_arrangements(&springs_groups).to_string();
}

#[derive(Debug, PartialEq, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}

fn parse_input(input: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
    let mut result = Vec::new();

    for line in input.lines() {
        // Split line at the space
        //println!("line: {}", line);
        let split_res = line.split_once(" ");
        let spring_str;
        let groups_str;

        if split_res.is_none() {
            spring_str = line.trim();
            groups_str = "";
        } else {
            spring_str = split_res.unwrap().0;
            groups_str = split_res.unwrap().1;
        }
        // let (spring_str, groups_str) = split_res.unwrap();

        let springs: Vec<Spring> = spring_str.chars().map(|c| match c {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => panic!("Invalid spring character")
        }).collect();

        let groups: Vec<usize>;
        
        if groups_str == "" {
            groups = Vec::new();
        } else {
            groups = groups_str.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        }

        result.push((springs, groups));
    }

    return result;
}

fn matches_pattern(springs: &Vec<Spring>, pattern: &Vec<Spring>) -> bool {
    if springs.len() != pattern.len() {
        return false;
    }

    for i in 0..pattern.len() {
        if pattern[i] == Spring::Unknown {
            continue;
        }

        if springs[i] != pattern[i] {
            return false;
        }
    }

    return true;
}

fn generate_arrangements(length: usize, groups: &Vec<usize>) -> Vec<Vec<Spring>> {
    // An arrangement may begin with a working spring, followed by the first group,
    // and then at least a working spring followed by the second group, then a
    // working spring et cetera.  All groups must be present, and there must be
    // working springs between all groups.

    // let indent = "  ".repeat(depth);
    // println!("{}generate_arrangements({}, {:?})", indent, length, groups);

    // print!("{}", indent);
    // println!(" length: {}", length);
    let mut result = Vec::new();

    let num_groups = groups.len();
    // print!("{}", indent);
    // println!(" num_groups: {}", num_groups);
    let spacers = num_groups - 1;
    // print!("{}", indent);
    // println!(" spacers: {}", spacers);
    let known_broken = groups.iter().sum::<usize>();
    // print!("{}", indent);
    // println!(" known_broken: {}", known_broken);
    let available_springs = length - known_broken - spacers;
    // print!("{}", indent);
    // println!(" available_springs: {}", available_springs);

    for i in 0..available_springs+1 {
        // print!("{}", indent);
        // println!(" i: {}", i);
        let mut springs = Vec::new();

        // Add working springs before the first group
        for _ in 0..i {
            springs.push(Spring::Operational);
        }

        // Add the first group
        for _ in 0..groups[0] {
            springs.push(Spring::Damaged);
        }

        // print!("{}", indent);
        // println!("  springs so far: {:?}", springs);
        // print!("{}", indent);
        // println!("  num_groups: {}", num_groups);
        
        // Recursively add the remaining groups
        if num_groups > 1 {
            // Add spacer
            springs.push(Spring::Operational);

            let mut remaining_groups = groups.clone();
            remaining_groups.remove(0);
            let remaining_springs = length - springs.len();
            let remaining_arrangements = generate_arrangements(remaining_springs, &remaining_groups); //    , depth+2);
            // print!("{}", indent);
            // println!("  remaining_arrangements: {:?}", remaining_arrangements);
            for remaining_arrangement in remaining_arrangements {
                let mut springs = springs.clone();
                springs.extend(remaining_arrangement);

                for _ in 0..length - springs.len() {
                    springs.push(Spring::Operational);
                }

                // print!("{}", indent);
                // println!("  Adding: {:?}", springs);
                result.push(springs);
            }
        } else {
            for _ in 0..length - springs.len() {
                springs.push(Spring::Operational);
            }

            // print!("{}", indent);
            // println!("  Adding: {:?}", springs);
            result.push(springs);
        }
    }

    return result;
}

fn calculate_arrangements(springs_groups: &Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
    let mut result = 0;

    for (pattern, groups) in springs_groups {
        let mut arrangements = 0;
        let length = pattern.len();

        let possibilities = generate_arrangements(length, groups);
        for possible in possibilities {
            if matches_pattern(&possible, &pattern) {
                arrangements += 1;
            }
        }

        result += arrangements;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1";

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn parse_input_test_1() {
        let result = parse_input(INPUT1);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].0.len(), 7);
        assert_eq!(result[0].1.len(), 3);
        assert_eq!(result[1].0.len(), 14);
        assert_eq!(result[1].1.len(), 3);
        assert_eq!(result[2].0.len(), 15);
        assert_eq!(result[2].1.len(), 4);
        assert_eq!(result[3].0.len(), 13);
        assert_eq!(result[3].1.len(), 3);
        assert_eq!(result[4].0.len(), 19);
        assert_eq!(result[4].1.len(), 3);
        assert_eq!(result[5].0.len(), 12);
        assert_eq!(result[5].1.len(), 3);
    }

    #[test]
    fn parse_input_test() {
        let result = parse_input(INPUT);
        assert_eq!(result.len(), 6);
        assert_eq!(result[0].0.len(), 7);
        assert_eq!(result[0].1.len(), 3);
        assert_eq!(result[1].0.len(), 14);
        assert_eq!(result[1].1.len(), 3);
        assert_eq!(result[2].0.len(), 15);
        assert_eq!(result[2].1.len(), 4);
        assert_eq!(result[3].0.len(), 13);
        assert_eq!(result[3].1.len(), 3);
        assert_eq!(result[4].0.len(), 19);
        assert_eq!(result[4].1.len(), 3);
        assert_eq!(result[5].0.len(), 12);
        assert_eq!(result[5].1.len(), 3);

        // "????.######..#####. 1,6,5"
        assert_eq!(
            result[4].0,
            vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Operational,
            ]
        );
    }

    #[test]
    fn matches_pattern_test() {
        // fn matches_pattern(springs: &Vec<Spring>, pattern: &Vec<Spring>) -> bool

        let pattern = &parse_input("?###???????? 3,2,1")[0].0;

        let spring_strings = ".###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#";
        for (springs, _) in parse_input(spring_strings) {
            assert_eq!(matches_pattern(&springs, pattern), true);
        }
        
        let spring_strings = "..##.##.#...";
        for (springs, _) in parse_input(spring_strings) {
            assert_eq!(matches_pattern(&springs, pattern), false);
        }
    }

    #[test]
    fn generate_arrangements_test1() {
        let spring_strings = "###.##..
###..##.
###...##
.###.##.
.###..##
..###.##";
        let result = generate_arrangements(8, &vec![3, 2]); //, 0);
        assert_eq!(result.len(), 6);

        for (i, springs) in result.iter().enumerate() {
            println!("{}: {:?}", i, springs.iter().map(|s| match s {
                Spring::Operational => '.',
                Spring::Damaged => '#',
                Spring::Unknown => '?',
            }).collect::<String>());
        }


        for (i, springs) in result.iter().enumerate() {
            assert_eq!(springs, &result[i]);
        }
    }
    #[test]
    fn generate_arrangements_test2() {
        let spring_strings = "###.##.#....
###.##..#...
###.##...#..
###.##....#.
###.##.....#
###..##.#...
###..##..#..
###..##...#.
###..##....#
###...##.#..
###...##..#.
###...##...#
###....##.#.
###....##..#
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#
..###.##.#..
..###.##..#.
..###.##...#
..###..##.#.
..###..##..#
..###...##.#
...###.##.#.
...###.##..#
...###..##.#
....###.##.#";
        let result = generate_arrangements(12, &vec![3, 2, 1]); // , 0);
        assert_eq!(result.len(), 35);

        for (i, springs) in result.iter().enumerate() {
            println!("{}: {:?}", i, springs.iter().map(|s| match s {
                Spring::Operational => '.',
                Spring::Damaged => '#',
                Spring::Unknown => '?',
            }).collect::<String>());
        }

        for (i, springs) in result.iter().enumerate() {
            assert_eq!(springs, &result[i]);
        }
    }

    #[test]
    fn filter_arrangements_test() {
        let (pattern, groups) = &parse_input("?###???????? 3,2,1")[0];
        let length = pattern.len();

        let possibilities = generate_arrangements(length, groups);
        println!("possibilities: {}", possibilities.len());

        let mut arrangements = 0;
        println!("pattern:  {:?}", pattern);
        for possible in possibilities {
            println!("possible: {:?}", possible);
            if matches_pattern(&possible, &pattern) {
                arrangements += 1;
            }
        }

        assert_eq!(arrangements, 10);
    }

    #[test]
    fn calculate_arrangements_test() {
        let springs_groups = parse_input(INPUT);
        let result = calculate_arrangements(&springs_groups);
        assert_eq!(result, 21);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "21".to_string());
    }
}