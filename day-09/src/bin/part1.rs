fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_input(input: &str) -> Vec<i64> {
    return input.split_whitespace().map(|word| {
        return word.parse::<i64>().unwrap();
    }).collect();
}

fn reduce_row(row: &Vec<i64>) -> Vec<i64> {
    let mut deltas: Vec<i64> = Vec::new();
    
    for i in 0..row.len()-1 {
        let a = row[i];
        let b = row[i+1];
        deltas.push(b - a);
    }
    return deltas;
}

fn reduce_all(row: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    rows.push(row.clone());
    while !is_zero_row(&rows[rows.len()-1]) {
        rows.push(reduce_row(&rows[rows.len()-1]));
    }
    return rows;
}

fn predict(deltas: &Vec<i64>, old_row: &Vec<i64>) -> i64 {
    return old_row[old_row.len()-1] + deltas[deltas.len()-1];
}

fn predict_all(old_rows: &Vec<Vec<i64>>) -> i64 {
    let mut addition: i64 = 0;
    for row in old_rows.iter().rev() {
        addition += row[row.len()-1];
    }
    return addition;
}

fn is_zero_row(row: &Vec<i64>) -> bool {
    for i in 0..row.len() {
        if row[i] != 0 {
            return false;
        }
    }
    return true;
}

fn part1(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let value = predict_all(&reduce_all(&parse_input(line)));
        total += value;
    }

    return total.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn parse_input_test() {
        let mut result: Vec<Vec<i64>> = Vec::new();
        for line in INPUT.lines() {
            result.push(parse_input(line));
        }
        assert_eq!(result, vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]);
    }

    #[test]
    fn reduce_row_test() {
        let result = reduce_row(&vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, vec![3, 3, 3, 3, 3]);
    }

    #[test]
    fn reduce_all_test() {
        let result = reduce_all(&vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result, vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ]);
    }

    #[test]
    fn predict_test() {
        let deltas = vec![3, 3, 3, 3, 3];
        let old_row = vec![0, 3, 6, 9, 12, 15];
        let result = predict(&deltas, &old_row);
        assert_eq!(result, 18);
    }

    #[test]
    fn predict_all_test() {
        let old_rows = vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![3, 3, 3, 3, 3],
            vec![0, 0, 0, 0],
        ];
        let result = predict_all(&old_rows);
        assert_eq!(result, 18);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "114".to_string());
    }
}