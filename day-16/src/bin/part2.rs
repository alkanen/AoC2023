const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

const EMPTY: char = '.';
const HORIZONTAL: char = '-';
const VERTICAL: char = '|';
const TOP_RIGHT: char = '/';
const TOP_LEFT: char = '\\';

#[derive(Clone)]
pub struct Field {
    tiles: Vec<String>,
    beams: Vec<Vec<usize>>,
    ins: Vec<Vec<Vec<(i32, i32)>>>,
}

impl Field {
    fn new(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut beams = Vec::new();
        // Record of incoming beams
        let mut ins = Vec::new();
        for line in input.lines() {
            tiles.push(line.to_string());

            // Create a line of 0 counters for each line of tiles
            beams.push(vec![0; line.len()]);

            // Create a line of empty lists for each line of tiles
            ins.push(vec![Vec::new(); line.len()]);
        }
        return Self {
            tiles: tiles,
            beams: beams,
            ins: ins,
        }
    }

    fn traverse(&mut self, start: (i32, i32), direction: (i32, i32))
    {
        let mut direction = direction.clone();
        let mut position: (i32, i32) = start.clone();

        loop {

            let next_pos = (position.0 + direction.0, position.1 + direction.1);
            if (next_pos.0 < 0 || next_pos.1 < 0)
                || (next_pos.0 >= self.tiles[0].len() as i32 || next_pos.1 >= self.tiles.len() as i32
            ) {
                // We've reached the end
                break;
            }

            let next_tile = self.get_tile(next_pos.0 as usize, next_pos.1 as usize);
            self.beams[next_pos.1 as usize][next_pos.0 as usize] += 1;

            if self.add_direction(next_pos.0 as usize, next_pos.1 as usize, direction) {
                // println!("Already used direction {:?} at position {:?}", direction, next_pos);
                break;
            }

            let done: bool;
            
            match next_tile {
                EMPTY => {
                    // Continue in same direction
                    done = false;
                },
                HORIZONTAL => {
                    // Beam coming down splits left and right
                    // Beam coming up splits left and right
                    if direction == DOWN ||direction == UP {
                        self.traverse(next_pos, LEFT);
                        self.traverse(next_pos, RIGHT);

                        done = true;
                    } else {
                        // Beam coming left goes left
                        // Beam coming right goes right
                        done = false;
                    }
                },
                VERTICAL => {
                    // Beam coming from left splits up and down
                    // Beam coming from right splits up and down
                    if direction == LEFT || direction == RIGHT {
                        self.traverse(next_pos, UP);
                        self.traverse(next_pos, DOWN);

                        done = true;
                    } else {
                        // Beam coming up goes up
                        // Beam coming down goes down
                        done = false;
                    }
                },
                TOP_RIGHT => {
                    // Beam coming up goes right
                    if direction == UP {
                        direction = RIGHT;
                        done = false;
                    }

                    // Beam coming right goes up
                    else if direction == RIGHT {
                        direction = UP;
                        done = false;
                    }

                    // Beam coming down goes left
                    else if direction == DOWN {
                        direction = LEFT;
                        done = false;
                    }

                    // Beam coming left goes down
                    else if direction == LEFT {
                        direction = DOWN;
                        done = false;
                    }

                    else {
                        done = false;
                    }
                },
                TOP_LEFT => {
                    // Beam coming up goes left
                    if direction == UP {
                        direction = LEFT;
                        done = false;
                    }

                    // Beam coming left goes up
                    else if direction == LEFT {
                        direction = UP;
                        done = false;
                    }

                    // Beam coming down goes right
                    else if direction == DOWN {
                        direction = RIGHT;
                        done = false;
                    }

                    // Beam coming right goes down
                    else if direction == RIGHT {
                        direction = DOWN;
                        done = false;
                    }

                    else {
                        done = false;
                    }
                },
                _ => {
                    panic!("Unknown tile: {}", next_tile);
                }
            };

            if done {
                break;
            }

            position = next_pos;
        };
    }

    fn get_tile(&self, x: usize, y: usize) -> char {
        // Bounds check is up to the caller
        return self.tiles[y].chars().nth(x).unwrap();
    }

    #[cfg(test)]
    fn get_beams_string(&self) -> String {
        let mut result = String::new();
        for (_y, line) in self.beams.iter().enumerate() {
            for (_x, beam) in line.iter().enumerate() {
                // result.push_str(&format!("{}{:2}  ", self.get_tile(x, y), beam));
                if *beam == 0 {
                    result.push_str(".");
                } else {
                    result.push_str("#");
                }
            }
            result.push_str("\n");
        }
        return result;
    }

    fn add_direction(&mut self, x: usize, y: usize, direction: (i32, i32)) -> bool {
        // Returns true if direction has already been used

        let used: bool = self.ins[y][x].contains(&direction);

        self.ins[y][x].push(direction);

        return used;
    }

    fn count_energized(&self) -> usize {
        let mut count = 0;
        for line in &self.beams {
            for beam in line {
                if *beam > 0 {
                    count += 1;
                }
            }
        }
        return count;
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let orig = parse(input);

    let mut best_count = 0;

    for y in 0..orig.tiles.len() {
        let x = -1;
        let mut contraption = orig.clone();
        contraption.traverse((x, y as i32), RIGHT);
        let count = contraption.count_energized();
        if count > best_count {
            best_count = count;
        }

        let x = orig.tiles[0].len() as i32;
        let mut contraption = orig.clone();
        contraption.traverse((x, y as i32), LEFT);
        let count = contraption.count_energized();
        if count > best_count {
            best_count = count;
        }
    }

    for x in 0..orig.tiles[0].len() {
        let y = -1;
        let mut contraption = orig.clone();
        contraption.traverse((x as i32, y), DOWN);
        let count = contraption.count_energized();
        if count > best_count {
            best_count = count;
        }

        let y = orig.tiles.len() as i32;
        let mut contraption = orig.clone();
        contraption.traverse((x as i32, y), UP);
        let count = contraption.count_energized();
        if count > best_count {
            best_count = count;
        }
    }

    return best_count.to_string();
}

fn parse(input: &str) -> Field {
    return Field::new(input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_parse() {
        let result = parse(INPUT);
        assert_eq!(result.tiles.len(), 10);
        assert_eq!(result.tiles[0].len(), 10);
    }

    #[test]
    fn test_traverse() {
        let mut contraption = parse(INPUT);
        contraption.traverse((3, -1), DOWN);
        println!("{}", contraption.get_beams_string());
        assert_eq!(
            contraption.get_beams_string(),
            ".#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
"
        );
    }

    #[test]
    fn test_count_energized() {
        let mut contraption = parse(INPUT);
        contraption.traverse((3, -1), DOWN);
        assert_eq!(contraption.count_energized(), 51);
    }

    #[test]
    fn it_works1() {
        let result = part2(INPUT);
        assert_eq!(result, "51".to_string());
    }
}