use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0, space0},
    combinator::{map, map_res, opt},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut system = System::new(input);

    for _ in 0..1000 {
        system.push_button();
    }
    
    return (system.count_high * system.count_low).to_string();
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
    Broadcast,
    Flipflop,
    Conjunction,
    Button,
    Placeholder,
}

#[derive(Debug)]
struct Pulse {
    source: String,
    target: String,
    value: bool,
}

#[derive(Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    // String is the name of the input module, bool is last known value,
    // true = high, false = low.
    inputs: HashMap<String, bool>,
    targets: Vec<String>,
    output: bool,
}
impl Module {
    fn new(name: String, module_type: ModuleType) -> Self {
        Self {
            name,
            module_type,
            // Inputs aren't done in order so use a hash
            inputs: HashMap::new(),
            // Outputs have to be sent in the right order, so use a vector
            targets: Vec::new(),
            // The current output value for this module
            output: false,
        }
    }

    fn add_input(&mut self, name: String) {
        self.inputs.insert(name, false);
    }

    fn add_target(&mut self, name: String) {
        self.targets.push(name);
    }

    fn recieve_signal(&mut self, signal: &Pulse, queue: &mut Vec<Pulse>) {
        // println!("recieve_signal() {} -{} -> {}", signal.source, match signal.value { true => {"high"}, false => {"low"}}, signal.target);
        //self.inputs[&signal.source] = signal.value;
        *self.inputs.get_mut(&signal.source).unwrap() = signal.value;

        match self.module_type {
            ModuleType::Broadcast => {
                self.output = signal.value;
            },
            ModuleType::Flipflop => {
                if signal.value {
                    return;
                }
                self.output = !self.output;
            },
            ModuleType::Conjunction => {
                // println!("  Conjunection {} inputs: {:?}", self.name, self.inputs);
                // If all inputs are true, set output to false
                // If any input is false, set output to true
                self.output = false;
                for (_, value) in self.inputs.iter() {
                    if !value {
                        self.output = true;
                        break;
                    }
                }
            },
            ModuleType::Button => {
                self.output = false;
            },

            ModuleType::Placeholder => {
                self.output = signal.value;
            },
        }

        for target in self.targets.iter() {
            // println!("{} -{} -> {}", self.name, match self.output { true => {"high"}, false => {"low"}}, target);
            queue.push(Pulse {
                source: self.name.clone(),
                target: target.clone(),
                value: self.output,
            });
        }
    }
}

#[derive(Debug)]
struct System {
    modules: HashMap<String, Module>,
    count_high: usize,
    count_low: usize,
}
impl System {
    fn new(input: &str) -> Self {
        Self {
            modules: parse(input),
            count_high: 0,
            count_low: 0,
        }
    }
    fn push_button(&mut self) {
        let module = self.modules.get_mut("broadcaster").unwrap();
        let mut queue: Vec<Pulse> = Vec::new();
    
        /*
        module.recieve_signal(
            &Pulse {
                source: "button".to_string(),
                target: module.name.clone(),
                value: false,
            },
            &mut queue
        );
        */
    
        let mut loop_ctr = 0;

        queue.push(Pulse {
            source: "button".to_string(),
            target: module.name.clone(),
            value: false,
        });
    
        loop {
            // println!("loop_ctr: {}", loop_ctr);
            loop_ctr += 1;
            let mut new_queue: Vec<Pulse> = Vec::new();
    
            for pulse in queue.iter() {
                if pulse.value {
                    self.count_high += 1;
                } else {
                    self.count_low += 1;
                }

                let module = self.modules.get_mut(&pulse.target).unwrap();
                module.recieve_signal(pulse, &mut new_queue);
            }
    
            /*
            println!("  Current state:");
            for (name, module) in self.modules.iter() {
                println!("    {}: {}", name, module.output);
            }
            */
    
            if new_queue.len() == 0 {
                // println!("  No more pulses to process");
                break;
            }
    
            queue = new_queue;
    
            /*
            println!("");
            println!("  queue:\n    {:?}", queue);
            */
        }
    }
}

fn parse_lines(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();

    for line in input.lines() {
        let (_, module) = parse_line(line).unwrap();
        modules.insert(module.name.clone(), module);
    }

    return modules;
}

fn set_target_inputs(modules: &mut HashMap<String, Module>, input: String, targets: &Vec<String>) {
    for target in targets.iter() {
        if !modules.contains_key(target) {
            let mut t = Module::new(target.clone(), ModuleType::Placeholder);
            t.add_input(input.clone());
            modules.insert(target.clone(), t);
            continue;
        }

        let mut t = modules.get_mut(target).unwrap();
        t.add_input(input.clone());
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut modules = parse_lines(input);

    let mut inputs: Vec<(String, Vec<String>)> = Vec::new();
    for (_, module) in modules.iter_mut() {
        let name = module.name.clone();
        let targets = module.targets.clone();

        inputs.push((name, targets));
    }

    for (name, targets) in inputs.iter() {
        set_target_inputs(&mut modules, name.clone(), targets);
    }
    set_target_inputs(&mut modules, "button".to_string(), &vec!["broadcaster".to_string()]);

    return modules;
}

fn parse_line(input: &str) -> IResult<&str, Module> {
    let (input, module_type) = alt((
        map(tag("%"), |_| ModuleType::Flipflop),
        map(tag("&"), |_| ModuleType::Conjunction),
        map(space0, |_| ModuleType::Broadcast),
    ))(input)?;

    let (input, (name, _, targets)) = tuple((
        alpha1,
        tag(" -> "),
        separated_list1(tag(", "), alpha1),
    ))(input)?;

    println!("module_type: {:?}", module_type);
    println!("name: {:?}", name);
    println!("targets: {:?}", targets);

    let mut module = Module::new(name.to_string(), module_type);
    for target in targets.iter() {
        module.add_target(target.to_string());
    }

    return Ok((input, module));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const INPUT: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_parse_line() {
        let result = parse_line("broadcast -> a, b, c").unwrap();
        let module = result.1;
        assert_eq!(module.name, "broadcast".to_string());
        assert_eq!(module.module_type, ModuleType::Broadcast);
        assert_eq!(module.targets, vec!["a".to_string(), "b".to_string(), "c".to_string()]);

        let result = parse_line("%a -> b").unwrap();
        let module = result.1;
        assert_eq!(module.name, "a".to_string());
        assert_eq!(module.module_type, ModuleType::Flipflop);
        assert_eq!(module.targets, vec!["b".to_string()]);

        let result = parse_line("&inv -> a").unwrap();
        let module = result.1;
        assert_eq!(module.name, "inv".to_string());
        assert_eq!(module.module_type, ModuleType::Conjunction);
        assert_eq!(module.targets, vec!["a".to_string()]);
    }

    #[test]
    fn test_parse() {
        let result = parse(INPUT1);
        assert_eq!(result.len(), 5);
        assert_eq!(result["broadcaster"].name, "broadcaster".to_string());
        assert_eq!(result["broadcaster"].module_type, ModuleType::Broadcast);
        assert_eq!(result["broadcaster"].targets, vec!["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(result["broadcaster"].inputs.len(), 1);
        assert_eq!(result["a"].name, "a".to_string());
        assert_eq!(result["a"].module_type, ModuleType::Flipflop);
        assert_eq!(result["a"].targets, vec!["b".to_string()]);
        assert_eq!(result["b"].name, "b".to_string());
        assert_eq!(result["b"].module_type, ModuleType::Flipflop);
        assert_eq!(result["b"].targets, vec!["c".to_string()]);
        assert_eq!(result["c"].name, "c".to_string());
        assert_eq!(result["c"].module_type, ModuleType::Flipflop);
        assert_eq!(result["c"].targets, vec!["inv".to_string()]);
        assert_eq!(result["inv"].name, "inv".to_string());
        assert_eq!(result["inv"].module_type, ModuleType::Conjunction);
        assert_eq!(result["inv"].targets, vec!["a".to_string()]);
    }

    /*
    #[test]
    fn test_push_button() {
        let mut modules = parse(INPUT);
        push_button(&mut modules);
        assert_eq!(modules["broadcaster"].output, false);
        assert_eq!(modules["a"].output, false);
        assert_eq!(modules["b"].output, false);
        assert_eq!(modules["inv"].output, true);
        assert_eq!(modules["output"].output, false);
    }
    */

    #[test]
    fn test_system_parse() {
        let system = System::new(INPUT);
        println!("system: {:?}", system);
        println!("system.modules: {:?}", system.modules);
        println!("system.modules[\"output\"]: {:?}", system.modules["output"]);
    }

    #[test]
    fn test_push_button1() {
        let mut system = System::new(INPUT1);
        system.push_button();
        assert_eq!(system.modules["broadcaster"].output, false);
        assert_eq!(system.modules["a"].output, false);
        assert_eq!(system.modules["b"].output, false);
        assert_eq!(system.modules["c"].output, false);
        assert_eq!(system.modules["inv"].output, true);
        assert_eq!(system.count_high, 4);
        assert_eq!(system.count_low, 8);
    }

    #[test]
    fn test_push_button() {
        let mut system = System::new(INPUT);

        println!(" ===================== PUSH 1 ===================== ");
        system.push_button();
        assert_eq!(system.modules["output"].output, false);
        assert_eq!(system.count_high, 4);
        assert_eq!(system.count_low, 4);
        
        println!(" ===================== PUSH 2 ===================== ");
        system.push_button();
        assert_eq!(system.modules["output"].output, true);
        assert_eq!(system.count_high, 6);
        assert_eq!(system.count_low, 8);

        println!(" ===================== PUSH 3 ===================== ");
        system.push_button();
        assert_eq!(system.modules["output"].output, true);
        assert_eq!(system.count_high, 9);
        assert_eq!(system.count_low, 13);

        println!(" ===================== PUSH 4 ===================== ");
        system.push_button();
        assert_eq!(system.modules["output"].output, true);
        assert_eq!(system.count_high, 11);
        assert_eq!(system.count_low, 17);
    }


    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "11687500".to_string());
    }
}