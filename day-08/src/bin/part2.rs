use std::collections::{
    HashMap,
    HashSet
};

// https://www.youtube.com/watch?v=Ph7xHhBfH0w
use nom::{
    // bytes::complete::tag,
    sequence::tuple,
    character::complete::alphanumeric1
};

use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
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
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (_, left, _, right, _)) = tuple((tag("("), alphanumeric1, tag(", "), alphanumeric1, tag(")")))(input)?;
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

fn factorize(n: usize) -> Vec<usize> {
    let mut factors: Vec<usize> = Vec::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        factors.push(n);
    }
    factors.sort();
    return factors;
}

fn part2(input: &str) -> String {
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

    // println!("{:?}", nodes);

    let mut steps = 0;
    let mut curr_nodes: Vec<&str> = nodes.iter().filter(|(name, _)| name.ends_with("A")).map(|(name, _)| *name).collect();
    let num_curr_nodes = curr_nodes.len();
    let mut curr_counts: Vec<usize> = vec![0; num_curr_nodes];

    println!("num_curr_nodes: {}", num_curr_nodes);
    println!("curr_nodes: {:?}", curr_nodes);

    let mut done = false;

    loop {
        for instruction in instructions.chars() {
            //println!("instruction: {}", instruction);

            let mut next_nodes: Vec<&str> = Vec::new();
            for (i, curr_node) in curr_nodes.iter().enumerate() {

                if !curr_node.ends_with("Z") {
                    let next_node: &str;
                    match instruction {
                        'L' => {
                            next_node = nodes.get(curr_node).unwrap().left;
                        },
                        'R' => {
                            next_node = nodes.get(curr_node).unwrap().right;
                        },
                        _ => panic!("Unknown instruction: {}", instruction),
                    }

                    next_nodes.push(next_node);

                    if next_node.ends_with("Z") {
                        curr_counts[i] = steps + 1;
                    }
                } else {
                    next_nodes.push(curr_node);
                }
            }
            steps += 1;
    
            curr_nodes = next_nodes;
            //num_curr_nodes = curr_nodes.len();
            //println!("num_curr_nodes: {}", num_curr_nodes);
            //println!("curr_nodes: {:?}", curr_nodes);
    
            if curr_nodes.iter().filter(|name| name.ends_with("Z")).count() == num_curr_nodes {
                done = true;
                break;
            }
        }

        if done {
            break;
        }
    }

    println!("curr_counts: {:?}", curr_counts);
    println!("curr_nodes: {:?}", curr_nodes);

    let mut prod: usize = 1;
    let mut factors: HashSet<usize> = HashSet::new();
    for count in curr_counts {
        let fs = factorize(count);
        for f in fs {
            if !factors.contains(&f) {
                factors.insert(f);
                prod *= f;
            }
        }
    }

    println!("Product is {}", prod);

    // start nodes: ["GGA", "BJA", "AAA", "XVA", "LTA", "DXA"]
    // curr_counts: [24253, 18113, 22411, 16271, 14429, 13201]
    // last nodes:  ["GSZ", "FCZ", "ZZZ", "QXZ", "RPZ", "GHZ"]

    return prod.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn fact_test() {
        let numbers: Vec<usize> = vec![
            24253, 18113, 22411, 16271, 14429, 13201
        ];

        for number in numbers {
            println!("{}: {:?}", number, factorize(number));
        }
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "6".to_string());
    }
}