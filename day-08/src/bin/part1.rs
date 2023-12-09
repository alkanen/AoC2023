use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    sequence::tuple,
    character::complete::alpha1
};

use nom_supreme::error::ErrorTree;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
struct NamedNode<'a> {
    name: &'a str,
    node: Node<'a>,
}

fn node_parser(input: &str) -> nom::IResult<&str, NamedNode, ErrorTree<&str>> {
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (_, left, _, right, _)) = tuple((tag("("), alpha1, tag(", "), alpha1, tag(")")))(input)?;
    Ok(
        (
            input,
            NamedNode {
                name: name,
                node: Node {
                    left: left,
                    right: right,
                }
            }
        )
    )
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap();
    println!("{}", instructions);

    // Skip the empty line
    lines.next();

    let mut nodes: HashMap<&str, Node> = HashMap::new();
    for line in lines {
        let (_, nn) = node_parser(line).unwrap();
        nodes.insert(nn.name, nn.node);
    }

    println!("{:?}", nodes);

    let mut steps = 0;
    let mut curr_node = "AAA";
    loop {
        for instruction in instructions.chars() {
            match instruction {
                'L' => {
                    curr_node = nodes.get(curr_node).unwrap().left;
                },
                'R' => {
                    curr_node = nodes.get(curr_node).unwrap().right;
                },
                _ => {
                    panic!("Unknown instruction: {}", instruction);
                }
            }
            steps += 1;
            println!("{}: {}", steps, curr_node);
            if curr_node == "ZZZ" {
                return steps.to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "6".to_string());
    }
}