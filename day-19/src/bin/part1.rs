use nom::{
    IResult,
    character::complete::{
        alpha1,
        // digit1,
        // space0,
        // alphanumeric1,
    },
    sequence::{
        delimited,
        preceded,
        terminated,
        tuple,
    },
    combinator::map,
    branch::alt,
    multi::{
        separated_list0,
        // separated_list1,
    },
};


fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(_input: &str) -> String {
    return "todo!".to_string();
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Operator {
    GreaterThan,
    LessThan,
    EqualTo
}
impl From<Operator> for std::cmp::Ordering {
    fn from(op: Operator) -> Self {
        match op {
            Operator::GreaterThan => std::cmp::Ordering::Greater,
            Operator::LessThan => std::cmp::Ordering::Less,
            Operator::EqualTo => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rule<'a> {
    name: &'a str,
    operator: Operator,
    value: u32,
    next: &'a str,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rating {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

use nom::character::complete;

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    println!("parse_rule: '{}'", input);
    let (input, name) = alpha1(input)?;
    println!("  name: {}", name);
    let (input, operator) = alt((
        map(complete::char('>'), |_| Operator::GreaterThan),
        map(complete::char('<'), |_| Operator::LessThan),
    ))(input)?;
    let (input, value) = complete::u32(input)?;
    let (input, _) = complete::char(':')(input)?;
    let (input, next) = alpha1(input)?;

    let rule: Rule = Rule {
        name,
        operator,
        value,
        next,
    };

    return Ok((input, rule));
}

fn parse_rules(input: &str) -> IResult<&str, Vec<Rule>> {
    // TODO: Replace this with a simple `rules_vec = input.split(",").collect()``
    // and parse them each individually, making sure to treat the last one differently
    
    let mut rules_list = input.split(",").collect::<Vec<&str>>();
    println!("rules_list: {:?}", rules_list);

    println!("Fallback input: '{}'", rules_list.last().unwrap());
    // Pop the last element from the vector into a fallback
    let (_, mut fallback) = terminated(
        alpha1,
        nom::bytes::complete::tag("}")
    )(rules_list.pop().unwrap())?;

    let mut rules_list = rules_list.iter().map(|rule| {
        let (_input, rule) = parse_rule(rule).unwrap();
        return rule;
    }).collect::<Vec<Rule>>();
    

    // Remove trailing '}' on fallback if necessary
    if fallback.ends_with("}") {
        fallback = &fallback[..fallback.len()-1];
    }

    rules_list.push(
        Rule {
            name: "fallback",
            operator: Operator::EqualTo,
            value: 0,
            next: fallback,
        }
    );

    println!("Returning rules_list: {:?}", rules_list);
    Ok(("}", rules_list))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = alpha1(input)?;
    println!("name: {}, remains: '{}'", name, input);
    let (input, rules_list) = delimited(
        nom::bytes::complete::tag("{"),
        parse_rules,
        nom::bytes::complete::tag("}")
    )(input)?;

    let wf: Workflow = Workflow {
        name,
        rules: rules_list,
    };

    return Ok((input, wf));
}

fn parse_rating(input: &str) -> IResult<&str, Rating> {
    let (input, rating) = delimited(
        nom::bytes::complete::tag("{"),
        separated_list0(
            nom::bytes::complete::tag(","),
            tuple((
                alpha1,
                preceded(
                    nom::bytes::complete::tag("="),
                    complete::u32,
                ),
            ))
        ),
        nom::bytes::complete::tag("}")
    )(input)?;

    let rating: Rating = Rating {
        x: rating[0].1,
        m: rating[1].1,
        a: rating[2].1,
        s: rating[3].1,
    }; 

    return Ok((input, rating));
}

fn parse(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Rating>)> {
    // First parse multiple lines of Workflows, then
    // a single empty line followed by multiple lines of Ratings

    let mut workflows: Vec<Workflow> = Vec::new();
    let mut ratings: Vec<Rating> = Vec::new();

    let mut workflows_done = false;
    for line in input.lines() {
        if line.is_empty() {
            workflows_done = true;
            continue;
        }

        if workflows_done {
            let (_, rating) = parse_rating(line)?;
            ratings.push(rating);
            continue;
        } else {
            let (_, workflow) = parse_workflow(line)?;
            workflows.push(workflow);
        }
    }

    return Ok((input, (workflows, ratings)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parse_workflow() {
        let result = parse_workflow("px{a<2006:qkq,m>2090:A,rfg}");
        dbg!(&result);
        assert!(result.is_ok());
        let (_, rule) = result.unwrap();
        assert_eq!(rule.name, "px");
    }

    #[test]
    fn test_parse_rating() {
        let result = parse_rating("{x=787,m=2655,a=1222,s=2876}");
        dbg!(&result);
        assert!(result.is_ok());
        let (_, rating) = result.unwrap();
        assert_eq!(rating.x, 787);
        assert_eq!(rating.m, 2655);
        assert_eq!(rating.a, 1222);
        assert_eq!(rating.s, 2876);
    }

    #[test]
    fn test_parse_everything() {
        let result = parse(INPUT);
        dbg!(&result);
        assert!(result.is_ok());
        let (_, (workflows, ratings)) = result.unwrap();
        assert_eq!(workflows.len(), 11);
        assert_eq!(ratings.len(), 5);

        // "px{a<2006:qkq,m>2090:A,rfg}"
        assert_eq!(workflows[0].name, "px");
        assert_eq!(workflows[0].rules.len(), 3);
        assert_eq!(workflows[0].rules[0].name, "a");
        assert_eq!(workflows[0].rules[0].operator, Operator::LessThan);
        assert_eq!(workflows[0].rules[0].value, 2006);
        assert_eq!(workflows[0].rules[0].next, "qkq");
        assert_eq!(workflows[0].rules[1].name, "m");
        assert_eq!(workflows[0].rules[1].operator, Operator::GreaterThan);
        assert_eq!(workflows[0].rules[1].value, 2090);
        assert_eq!(workflows[0].rules[1].next, "A");
        assert_eq!(workflows[0].rules[2].name, "fallback");
        assert_eq!(workflows[0].rules[2].operator, Operator::EqualTo);
        assert_eq!(workflows[0].rules[2].value, 0);
        assert_eq!(workflows[0].rules[2].next, "rfg");
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "todo!".to_string());
    }
}