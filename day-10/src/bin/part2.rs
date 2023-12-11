fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pipe {
    Empty=0,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pos {
    x: i64,
    y: i64,
}

fn pipe_to_char(pipe: Pipe) -> char {
    return match pipe {
        Pipe::Empty => '.',
        Pipe::Vertical => '|',
        Pipe::Horizontal => '-',
        Pipe::NorthEast => 'L',
        Pipe::NorthWest => 'J',
        Pipe::SouthEast => 'F',
        Pipe::SouthWest => '7',
        Pipe::Unknown => 'S',
    };
}

fn pipe_directions(pipe: Pipe) -> (Pos, Pos) {
    return match pipe {
        // Fail horribly
        Pipe::Empty => panic!("Empty pipe!"),
        Pipe::Vertical => (Pos{x: 0, y: 1}, Pos{x: 0, y: -1}),
        Pipe::Horizontal => (Pos{x: -1, y: 0}, Pos{x: 1, y: 0}),
        Pipe::NorthEast => (Pos{x: 1, y: 0}, Pos{x: 0, y: -1}),
        Pipe::NorthWest => (Pos{x: -1, y: 0}, Pos{x: 0, y: -1}),
        Pipe::SouthEast => (Pos{x: 0, y: 1}, Pos{x: 1, y: 0}),
        Pipe::SouthWest => (Pos{x: 0, y: 1}, Pos{x: -1, y: 0}),
        // Fail horribly
        Pipe::Unknown => panic!("Unknown pipe!"),
    };
}

fn parse_pipes(input: &str) -> (Vec<Vec<Pipe>>, Pos) {
    let mut pipes: Vec<Vec<Pipe>> = Vec::new();
    let mut animal_x: i64 = -1;
    let mut animal_y: i64 = -1;
    for (y, line) in input.lines().enumerate() {
        let mut row: Vec<Pipe> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                '.' => Pipe::Empty,
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NorthEast,
                'J' => Pipe::NorthWest,
                'F' => Pipe::SouthEast,
                '7' => Pipe::SouthWest,
                'S' => Pipe::Unknown,
                _ => panic!("Unknown character: {}", c),
            });

            if c == 'S' {
                animal_x = x as i64;
                animal_y = y as i64;
            }
        }
        pipes.push(row);
    }

    if animal_x == -1 || animal_y == -1 {
        println!("Animal not found!");
    } else {
        let x = animal_x as usize;
        let y = animal_y as usize;
        let mut north = false;
        let mut south = false;
        let mut east = false;
        let mut west = false;
        // If pipe above is vertical, southeast or soutwest, the tile has a pipe going north
        if y > 0 {
            let pipe_above = &pipes[y-1][x];
            if *pipe_above == Pipe::Vertical || *pipe_above == Pipe::SouthEast || *pipe_above == Pipe::SouthWest {
                north = true;
            }
        }

        // If pipe below is vertical, northeast or northwest, the tile has a pipe going south
        if y < pipes.len() - 1 {
            let pipe_below = &pipes[y+1][x];
            if *pipe_below == Pipe::Vertical || *pipe_below == Pipe::NorthEast || *pipe_below == Pipe::NorthWest {
                south = true;
            }
        }

        // If pipe left is horizontal, northeast or southeast, the tile has a pipe going west
        if x > 0 {
            let pipe_left = &pipes[y][x-1];
            if *pipe_left == Pipe::Horizontal || *pipe_left == Pipe::NorthEast || *pipe_left == Pipe::SouthEast {
                west = true;
            }
        }

        // If pipe right is horizontal, northwest or southwest, the tile has a pipe going east
        if x < pipes[0].len() - 1 {
            let pipe_right = &pipes[y][x+1];
            if *pipe_right == Pipe::Horizontal || *pipe_right == Pipe::NorthWest || *pipe_right == Pipe::SouthWest {
                east = true;
            }
        }
                
        if north && south && !(east || west) {
            pipes[y][x] = Pipe::Vertical;
        } else if east && west && !(north || south) {
            pipes[y][x] = Pipe::Horizontal;
        } else if north && east && !(south || west) {
            pipes[y][x] = Pipe::NorthEast;
        } else if north && west && !(south || east) {
            pipes[y][x] = Pipe::NorthWest;
        } else if south && east && !(north || west) {
            pipes[y][x] = Pipe::SouthEast;
        } else if south && west && !(north || east) {
            pipes[y][x] = Pipe::SouthWest;
        } else {
            println!("Animal is in a weird spot!");
        }
    }

    return (pipes, Pos { x: animal_x, y: animal_y });
}

fn visit_pipes(pipes: &Vec<Vec<Pipe>>, start: Pos) -> Vec<Vec<Pipe>> {
    let mut visited: Vec<Vec<Pipe>> = vec![vec![Pipe::Empty; pipes[0].len()]; pipes.len()];
    let mut pos = Pos{x: start.x, y: start.y};
    let mut p = pipes[pos.y as usize][pos.x as usize];

    visited[pos.y as usize][pos.x as usize] = pipes[pos.y as usize][pos.x as usize];

    let mut next_dir = pipe_directions(p).0;
    loop {
        let old_pos = Pos{x: pos.x, y: pos.y};

        // Move to next position
        pos.x += next_dir.x;
        pos.y += next_dir.y;

        if pos == start {
            // We're done
            break;
        }

        visited[pos.y as usize][pos.x as usize] = pipes[pos.y as usize][pos.x as usize];

        // Get pipe at new position
        p = pipes[pos.y as usize][pos.x as usize];

        // Get directions of pipe
        let dirs = pipe_directions(p);

        if (Pos{
            x: pos.x + dirs.0.x,
            y: pos.y + dirs.0.y
        }) == old_pos {
            next_dir = dirs.1;
        } else {
            next_dir = dirs.0;
        }
    }

    return visited;
}

fn update_state(state: &str, pipe: Pipe) -> (bool, &str) {
    let mut new_state: &str = "unknown";
    let mut cross: bool = false;

    if state == "outside" {
        if pipe == Pipe::Vertical {
            cross = true;
            new_state = "border";
        } else if pipe == Pipe::SouthEast {
            new_state = "if_next_up";
        } else if pipe == Pipe::NorthEast {
            new_state = "if_next_down";
        } else if pipe == Pipe::Empty {
            new_state = "outside";
        } else {
            panic!("outside, unexpected pipe: {}", pipe_to_char(pipe));
        }
    }

    return (cross, new_state);
}

fn calculate_enclosure(pipes: &Vec<Vec<Pipe>>) -> i64 {
    let mut enclosed = 0;

    let mut output: Vec<Vec<char>> = vec![Vec::new(); pipes.len()];

    for (y, row) in pipes.iter().enumerate() {
        let mut crosses = 0;
        let mut state = "outside";
        // println!("Row: {}", row.iter().map(|p| pipe_to_char(*p)).collect::<String>());

        for (x, pipe) in row.iter().enumerate() {
            let mut inside = false;
            crosses += match *pipe {
                Pipe::Vertical => 1,
                Pipe::SouthEast => 0,
                Pipe::NorthEast => 1,
                Pipe::SouthWest=> 0,
                Pipe::NorthWest => 1,
                Pipe::Empty => 0,
                _ => 0,
            };

            // println!("{}: {}", pipe_to_char(*pipe), crosses);
            if *pipe == Pipe::Empty && (crosses % 2) == 1 {
                output[y].push('I');
                enclosed += 1;
            } else {
                output[y].push(pipe_to_char(*pipe));
            }
        }

        println!("{}", output[y].iter().collect::<String>());
    }

    //panic!();

    return enclosed
}

fn part2(input: &str) -> String {
    let (pipes, pos) = parse_pipes(input);
    let visited = visit_pipes(&pipes, pos);
    let enclosed = calculate_enclosure(&visited);
    for row in visited {
        println!("{}", row.iter().map(|p| pipe_to_char(*p)).collect::<String>());
    }
    return enclosed.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    const INPUT_SIMPLER: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

    fn parse_pipe_test(input: &str, expected: (Vec<Vec<Pipe>>, Pos)) {
        let result = parse_pipes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_pipes_test1() {
        const INPUT1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

        parse_pipe_test(
            INPUT1,
            (
                vec![
                    vec![Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty],
                    vec![Pipe::Empty, Pipe::SouthEast, Pipe::Horizontal, Pipe::SouthWest, Pipe::Empty],
                    vec![Pipe::Empty, Pipe::Vertical, Pipe::Empty, Pipe::Vertical, Pipe::Empty],
                    vec![Pipe::Empty, Pipe::NorthEast, Pipe::Horizontal, Pipe::NorthWest, Pipe::Empty],
                    vec![Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty],
                ],
                Pos{x: 1, y: 1},
            )
        );
    }

    #[test]
    fn visited_test1() {
        const INP: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let (pipes, pos) = parse_pipes(INP);
        let visited = visit_pipes(&pipes, pos);

        for row in visited {
            println!("{}", row.iter().map(|p| pipe_to_char(*p)).collect::<String>());
        }
    }

    #[test]
    fn visited_test_simpler() {
        let (pipes, pos) = parse_pipes(INPUT_SIMPLER);
        let visited = visit_pipes(&pipes, pos);

        for row in visited {
            println!("{}", row.iter().map(|p| pipe_to_char(*p)).collect::<String>());
        }
    }


    #[test]
    fn enclosed_test1() {
        let (pipes, pos) = parse_pipes("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........");
        let visited = visit_pipes(&pipes, pos);
        let enclosed = calculate_enclosure(&visited);

        assert_eq!(enclosed, 4);
    }

    #[test]
    fn enclosed_test_simpler() {
        let (pipes, pos) = parse_pipes(INPUT_SIMPLER);
        let visited = visit_pipes(&pipes, pos);
        let enclosed = calculate_enclosure(&visited);

        assert_eq!(enclosed, 8);
        panic!();
    }


    /*
    #[test]
    fn it_works1() {
        let result = part2(INPUT);
        assert_eq!(result, "todo!".to_string());
    }
    */
}


/*

            if state == "outside" {
                if *pipe == Pipe::Vertical {
                    crosses += 1;
                    state = "inside";
                } else if *pipe == Pipe::SouthEast {
                    state = "if_next_up";
                } else if *pipe == Pipe::NorthEast {
                    state = "if_next_down";
                } else if *pipe == Pipe::Empty {
                    state = "outside";
                } else {
                    panic!("outside, unexpected pipe: {}", pipe_to_char(*pipe));
                }
            } else if state == "if_next_up" {
                if *pipe == Pipe::SouthWest {
                    state = "outside";
                    crosses += 1;
                } else if *pipe == Pipe::NorthWest {
                    state = "inside";
                    crosses += 1;
                } else if *pipe == Pipe::Horizontal {
                } else {
                    panic!("inu, unexpected pipe: {}", pipe_to_char(*pipe));
                }
            } else if state == "if_next_down" {
                if *pipe == Pipe::NorthWest {
                    state = "outside";
                    crosses += 1;
                } else if *pipe == Pipe::SouthWest {
                    state = "inside";
                    crosses += 1;
                } else if *pipe == Pipe::Horizontal {
                } else {
                    panic!("ind, unexpected pipe: {}", pipe_to_char(*pipe));
                }
            } else if state == "inside" {
                if *pipe == Pipe::Vertical {
                    state = "outside";
                    crosses += 1;
                } else if *pipe == Pipe::SouthWest {
                    state = "if_next_down";
                    crosses += 1;
                } else if *pipe == Pipe::NorthWest {
                    state = "if_next_up";
                    crosses += 1;
                } else if *pipe == Pipe::Empty {
                    state = "inside";
                    enclosed += 1;
                } else {
                    panic!("inside, unexpected pipe: {}", pipe_to_char(*pipe));
                }

            } else {
                panic!("Unknown state: {}", state);
            }
            */

            /*

            let cross = match *pipe {
                Pipe::Vertical => true,
                Pipe::SouthEast => false,
                Pipe::NorthEast => false,
                Pipe::Empty => false,
                _ => false,
            };

            inside = match *pipe {
                Pipe::Vertical => false,
                Pipe::SouthEast => false,
                Pipe::NorthEast => false,
                Pipe::Empty => true,
                _ => false,
            };

            if cross {
                crosses += 1;
            }

            if inside && (crosses % 2) == 1{
                output[y].push('+');
            } else {
                output[y].push(pipe_to_char(*pipe));
            }
            */