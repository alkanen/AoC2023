fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    return "todo!".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "todo!".to_string());
    }
}