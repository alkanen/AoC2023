use nom::{
    character::complete::{alpha1, digit1, one_of},
    combinator::{map_res, recognize},
    IResult,
};

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    input: String,
    label: String,
    operation: char,
    focal_length: u64,
}


struct Boxes {
    boxes: Vec<Vec<Instruction>>,
}
impl Boxes {
    fn new(size: usize) -> Self {
        let mut boxes: Vec<Vec<Instruction>> = Vec::new();
        for _ in 0..size {
            boxes.push(Vec::new());
        }
        return Self {
            boxes: boxes,
        }
    }

    fn add(&mut self, instr: Instruction) {
        let hash = do_hash(instr.label.as_str());

        if instr.operation == '-' {
            // Remove any instruction with the same label from the box
            let picked_box = &mut self.boxes[hash as usize];
            let mut index = 0;
            let mut found = false;
            for i in 0..picked_box.len() {
                if picked_box[i].label == instr.label {
                    index = i;
                    found = true;
                    break;
                }
            }
            if found {
                picked_box.remove(index);
            }
            return;
        } else if instr.operation == '=' {
            // Replace any instruction with the same label from the box,
            // or add it if it doesn't exist
            let picked_box = &mut self.boxes[hash as usize];
            let mut index = 0;
            let mut found = false;
            for i in 0..picked_box.len() {
                if picked_box[i].label == instr.label {
                    index = i;
                    found = true;
                    break;
                }
            }
            if found {
                picked_box[index] = Instruction {
                    input: instr.input.clone(),
                    label: instr.label.clone(),
                    operation: instr.operation.clone(),
                    focal_length: instr.focal_length.clone(),
                };
            } else {
                picked_box.push(Instruction {
                    input: instr.input.clone(),
                    label: instr.label.clone(),
                    operation: instr.operation.clone(),
                    focal_length: instr.focal_length.clone(),
                });
            }
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.boxes.len() {
            let picked_box = &self.boxes[i];
            result.push_str(&format!("Box {}: ", i));
            for instr in picked_box {
                result.push_str(&format!("[{} {}] ", instr.label, instr.focal_length));
            }
            result.push_str("\n");
        }
        return result;
    }

    fn compute_focusing_power(&self) -> u64 {
        let mut power: u64 = 0;
        for i in 0..self.boxes.len() {
            let picked_box = &self.boxes[i];
            for slot in 0..picked_box.len() {
                let instr = &picked_box[slot];

                power += (i as u64 + 1) * (slot as u64 + 1) * instr.focal_length as u64;
            }
        }

        return power;
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let boxes = follow_instructions(&parse(input));
    return boxes.compute_focusing_power().to_string();
}

fn my_u64(input : &str) -> IResult<&str, u64> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_entry(input: &str) -> IResult<&str, Instruction> {
    let orig_input = input;
    let (input, label) = alpha1(input)?;
    let (input, operation) = one_of("=-")(input)?;
    let value: u64;
    if operation == '-' {
        value = 0;
    } else {
        let (new_input, tmp_value) = my_u64(input)?;
        value = tmp_value;
    }

    return Ok((
        input,
        Instruction {
            input: orig_input.to_string(),
            label: label.to_string(),
            operation: operation as char,
            focal_length: value,
        }
    ));
}

fn parse(input: &str) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let parts = line.split(",");
        // Use nom to parse each part into a Instruction struct
        for part in parts {
            let (_, instruction) = parse_entry(part).unwrap();
            result.push(instruction);
        }
    }
    return result;
}

fn do_hash(input: &str) -> u64 {
    let mut result = 0;
    for c in input.bytes() {
        result += c as u64;
        result *= 17;
        result %= 256;
    }
    return result;
}


fn follow_instructions(instructions: &Vec<Instruction>) -> Boxes {
    let mut boxes: Boxes = Boxes::new(256);

    for instr in instructions {
        boxes.add(instr.clone());    
    }

    return boxes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    /*
    #[test]
    fn test_parse() {
        let result = parse(INPUT);
        //assert_eq!(result, vec!["rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"]);
    }
    */

    // Test cases: rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    #[test_log::test(rstest)]
    #[case("rn", 0)]
    #[case("cm-", 253)]
    #[case("qp", 1)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_do_hash(#[case] input: &str, #[case] expected: u64) {
        let result = do_hash(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_box_print() {
        let mut boxes = Boxes::new(3);
        boxes.add(Instruction {
            input: "rn=1".to_string(),
            label: "rn".to_string(),
            operation: '=' as char,
            focal_length: 1,
        });
        assert_eq!(boxes.to_string(), "Box 0: [rn 1] \nBox 1: \nBox 2: \n".to_string());
    }

    #[test]
    fn test_add_boxes() {
        let mut boxes = Boxes::new(4);
        let instructions = parse(INPUT);

        for instruction in instructions {
            // let local_input = instruction.input.clone();
            boxes.add(instruction);
            //println!("After \"{}\":", local_input);
            //println!("{}", boxes.to_string());
        }

        assert_eq!(
            boxes.to_string(),
             "Box 0: [rn 1] [cm 2] 
Box 1: 
Box 2: 
Box 3: [ot 7] [ab 5] [pc 6] 
".to_string());
    }

    #[test]
    fn test_power() {
        let mut boxes = Boxes::new(256);
        let instructions = parse(INPUT);

        for instruction in instructions {
            boxes.add(instruction);
        }

        assert_eq!(boxes.compute_focusing_power(), 145);
    }

    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "145".to_string());
    }
}