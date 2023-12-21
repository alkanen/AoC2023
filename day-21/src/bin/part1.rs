fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (plots, mut occupied, start) = parse(input);
    for i in 0..64 {
        occupied = step(&plots, &occupied);
    }

    return count_occupied(&occupied).to_string();
}

struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbours(&self, max_width: usize, max_height: usize) -> Vec<Pos> {
        let mut result = Vec::new();
        if self.x > 0 {
            result.push(Pos::new(self.x - 1, self.y));
        }
        if self.y > 0 {
            result.push(Pos::new(self.x, self.y - 1));
        }
        if self.x < max_width - 1 {
            result.push(Pos::new(self.x + 1, self.y));
        }
        if self.y < max_height - 1 {
            result.push(Pos::new(self.x, self.y + 1));
        }
        result
    }
}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Vec<bool>>, Pos) {
    // Returns:
    // - A grid of visitable positions
    // - A grid of currently occupied positions
    // - The starting position

    let mut plots = Vec::new();
    let mut occupied = Vec::new();
    let mut start = Pos::new(0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row_plots = Vec::new();
        let mut row_occupied = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    row_plots.push(true);
                    row_occupied.push(false);
                },
                '#' => {
                    row_plots.push(false);
                    row_occupied.push(false);
                },
                'S' => {
                    row_plots.push(true);
                    row_occupied.push(true);
                    start = Pos::new(x, y);
                }
                _ => panic!("unexpected char: {}", c),
            }
        }
        plots.push(row_plots);
        occupied.push(row_occupied);
    }
    (plots, occupied, start)
}

fn step(plots: &Vec<Vec<bool>>, occupied: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // Returns a new grid of occupied positions after one step.
    // Currently occupied positions are now longer occupied.
    // All orthogonal neighbours of currently occupied positions that are open plots are now occupied.
    let mut result = Vec::new();

    for (y, row) in occupied.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, is_occupied) in row.iter().enumerate() {
            if !plots[y][x] {
                new_row.push(false);
                continue;
            }

            if *is_occupied {
                new_row.push(false);
                continue;
            }

            let pos = Pos::new(x, y);
            let neighbours = pos.neighbours(plots[0].len(), plots.len());
            let mut is_occupied = false;
            for neighbour in neighbours {
                if occupied[neighbour.y][neighbour.x] {
                    is_occupied = true;
                    break;
                }
            }

            new_row.push(is_occupied);
        }
        result.push(new_row);
    }

    result
}

fn count_occupied(occupied: &Vec<Vec<bool>>) -> usize {
    let mut count = 0;
    for row in occupied.iter() {
        for is_occupied in row.iter() {
            if *is_occupied {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_parse() {
        let (plots, occupied, start) = parse(INPUT);
        assert_eq!(plots.len(), 11);
        assert_eq!(plots[0].len(), 11);
        assert_eq!(plots[10].len(), 11);
        assert_eq!(occupied.len(), 11);
        assert_eq!(occupied[0].len(), 11);
        assert_eq!(occupied[10].len(), 11);

        assert_eq!(start.x, 5);
        assert_eq!(start.y, 5);

        assert_eq!(plots[start.y][start.x], true);
        assert_eq!(occupied[start.y][start.x], true);
        for y in 0..11 {
            for x in 0..11 {
                if x == start.x && y == start.y {
                    continue;
                }
                assert_eq!(occupied[y][x], false);
            }
        }
    }

    #[test]
    fn test_step() {
        let (plots, occupied, start) = parse(INPUT);
        let occupied = step(&plots, &occupied);
        assert_eq!(occupied.len(), 11);
        assert_eq!(occupied[0].len(), 11);
        assert_eq!(occupied[10].len(), 11);

        assert_eq!(occupied[start.y][start.x], false);
        assert_eq!(occupied[start.y - 1][start.x], true && plots[start.y - 1][start.x]);
        assert_eq!(occupied[start.y + 1][start.x], true && plots[start.y + 1][start.x]);
        assert_eq!(occupied[start.y][start.x - 1], true && plots[start.y][start.x - 1]);
        assert_eq!(occupied[start.y][start.x + 1], true && plots[start.y][start.x + 1]);

        assert_eq!(count_occupied(&occupied), 2);
    }

    #[test]
    fn test_two_steps() {
        let (plots, occupied, start) = parse(INPUT);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);

        assert_eq!(count_occupied(&occupied), 4);
    }

    #[test]
    fn test_three_steps() {
        let (plots, occupied, start) = parse(INPUT);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);

        assert_eq!(count_occupied(&occupied), 6);
    }

    #[test]
    fn test_six_steps() {
        let (plots, occupied, start) = parse(INPUT);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);
        let occupied = step(&plots, &occupied);

        assert_eq!(count_occupied(&occupied), 16);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "todo!".to_string());
    }
}