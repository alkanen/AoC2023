fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let inputs = parse(input);
    let mut sum = 0;
    for input in inputs {
        sum += do_hash(input);
    }
    return sum.to_string();
}

fn parse(input: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();
    for line in input.lines() {
        let parts = line.split(",");
        // Extend the results vector with all values in parts:
        result.extend(parts);
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_parse() {
        let result = parse(INPUT);
        assert_eq!(result, vec!["rn=1", "cm-", "qp=3", "cm=2", "qp-", "pc=4", "ot=9", "ab=5", "pc-", "pc=6", "ot=7"]);
    }

    // Test cases: rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    #[test_log::test(rstest)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
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
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "1320".to_string());
    }
}