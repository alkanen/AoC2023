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

use indicatif::ProgressBar;
use memoize::memoize;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(_input: &str) -> String {

    return "todo!".to_string();
}

const UNKNOWN: char = '?';
const DAMAGED: char = '#';
const OPERATIONAL: char = '.';

#[derive(Clone, Debug, PartialEq)]
struct Entry {
    pattern: String,
    groups_string: String,
    groups: Vec<usize>,
}

#[memoize]
fn groups_string_to_numbers(groups_string: String) -> Vec<usize> {
    let mut result = Vec::new();
    for group in groups_string.split(',') {
        result.push(group.parse::<usize>().unwrap());
    }
    return result;
}

#[memoize]
fn generate_arrangements(length: usize, pattern: String) -> Vec<String> {
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
            springs.push(OPERATIONAL);
        }

        // Add the first group
        for _ in 0..groups[0] {
            springs.push(DAMAGED);
        }

        // print!("{}", indent);
        // println!("  springs so far: {:?}", springs);
        // print!("{}", indent);
        // println!("  num_groups: {}", num_groups);
        
        // Recursively add the remaining groups
        if num_groups > 1 {
            // Add spacer
            springs.push(OPERATIONAL);

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
                    springs.push(OPERATIONAL);
                }

                // print!("{}", indent);
                // println!("  Adding: {:?}", springs);
                result.push(springs);
            }
        } else {
            for _ in 0..length - springs.len() {
                springs.push(OPERATIONAL);
            }

            // print!("{}", indent);
            // println!("  Adding: {:?}", springs);
            result.push(springs.join(""));
        }
    }

    return result;
}

fn parse_input(string: &str) -> Vec<Entry> {
    let mut result = Vec::new();
    for line in string.lines() {
        let (pattern, groups_string) = line.split_once(" ").unwrap();
        //let line_parts: Vec<&str> = line.split_once(" ").collect();
        //let pattern: &str = line_parts[0];
        //let groups_string: &str = line_parts[1];
        let groups = groups_string_to_numbers(groups_string.to_string());

        println!("pattern: {}, groups_string: {}, groups: {:?}", pattern, groups_string, groups);

        let entry = Entry {
            pattern: pattern.to_string(),
            groups_string: groups_string.to_string(),
            groups: groups.clone(),
        };

        result.push(entry);
    }

    return result;
}

// #[memoize]

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_parse_input() {
        let result = parse_input(INPUT);
        assert_eq!(result.len(), 6);
        panic!();
    }

    #[test]
    fn test_generate_arrangements() {
        let result = generate_arrangements(10, "1,1,3".to_string());
        assert_eq!(result.len(), 20);
        assert_eq!(result[0], "#.#.###...".to_string());
        assert_eq!(result[1], "#.#..###..".to_string());
        assert_eq!(result[2], "#.#...###.".to_string());
        assert_eq!(result[3], "#.#....###".to_string());
        assert_eq!(result[4], "#..#.###..".to_string());
        assert_eq!(result[5], "#..#..###.".to_string());
        assert_eq!(result[6], "#..#...###".to_string());
        assert_eq!(result[7], "#...#.###.".to_string());
        assert_eq!(result[8], "#...#..###".to_string());
        assert_eq!(result[9], "#....#.###".to_string());
        assert_eq!(result[10], ".#.#.###..".to_string());
        assert_eq!(result[11], ".#.#..###.".to_string());
        assert_eq!(result[12], ".#.#...###".to_string());
        assert_eq!(result[13], ".#..#.###.".to_string());
        assert_eq!(result[14], ".#..#..###".to_string());
        assert_eq!(result[15], ".#...#.###".to_string());
        assert_eq!(result[16], "..#.#.###.".to_string());
        assert_eq!(result[17], "..#.#..###".to_string());
        assert_eq!(result[18], "..#..#.###".to_string());
        assert_eq!(result[19], "...#.#.###".to_string());
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "todo!".to_string());
    }
}