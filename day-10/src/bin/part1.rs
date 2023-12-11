fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
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

    return (pipes, Pos{x: animal_x, y: animal_y});
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

fn update_distances(pipes: &Vec<Vec<Pipe>>, start: Pos, direction: i32, distances: &mut Vec<Vec<i64>>) -> i64 {
    let mut pos = Pos{x: start.x, y: start.y};
    let mut p = pipes[pos.y as usize][pos.x as usize];

    let dists = pipe_directions(p);
    let mut next_dir;

    if direction == 0 {
        next_dir = dists.0;
    } else {
        next_dir = dists.1;
    }

    let mut dist = 0;
    loop {
        let old_pos = Pos{x: pos.x, y: pos.y};

        // Move to next position
        pos.x += next_dir.x;
        pos.y += next_dir.y;

        if pos == start {
            // We're done
            break;
        }

        dist += 1;
        if distances[pos.y as usize][pos.x as usize] != -1 && distances[pos.y as usize][pos.x as usize] <= dist {
            // We've already been here, and we've already found a shorter path
            break;
        }
        distances[pos.y as usize][pos.x as usize] = dist;

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

    return dist;
}

fn calculate_distances(pipes: &Vec<Vec<Pipe>>, start: Pos) -> (i64, Vec<Vec<i64>>) {
    let mut distances: Vec<Vec<i64>> = vec![vec![-1; pipes[0].len()]; pipes.len()];

    let pos = Pos{x: start.x, y: start.y};
    distances[pos.y as usize][pos.x as usize] = 0;

    update_distances(pipes, start, 0, &mut distances);
    let dist = update_distances(pipes, start, 1, &mut distances);

    println!("dist: {}", dist);

    return (dist, distances);
}

fn part1(input: &str) -> String {
    let (pipes, pos) = parse_pipes(input);
    let (dist, _distances) = calculate_distances(&pipes, pos);
    return dist.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = ".....
.S-7.
.|.|.
.L-J.
.....";

    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    fn parse_pipe_test(input: &str, expected: (Vec<Vec<Pipe>>, Pos)) {
        let result = parse_pipes(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_pipes_test1() {
        /*
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        */
        parse_pipe_test(INPUT1, (vec![
            vec![Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty],
            vec![Pipe::Empty, Pipe::SouthEast, Pipe::Horizontal, Pipe::SouthWest, Pipe::Empty],
            vec![Pipe::Empty, Pipe::Vertical, Pipe::Empty, Pipe::Vertical, Pipe::Empty],
            vec![Pipe::Empty, Pipe::NorthEast, Pipe::Horizontal, Pipe::NorthWest, Pipe::Empty],
            vec![Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty, Pipe::Empty],
        ], Pos{x: 1, y: 1}));
    }

    #[test]
    fn distance_grid_test1() {
        let (pipes, pos) = parse_pipes(".....
.S-7.
.|.|.
.L-J.
.....");
        let distances = calculate_distances(&pipes, pos);
        assert_eq!(
            distances,
            (
                4,
                vec![
                    vec![-1, -1, -1, -1, -1],
                    vec![-1,  0,  1,  2, -1],
                    vec![-1,  1, -1,  3, -1],
                    vec![-1,  2,  3,  4, -1],
                    vec![-1, -1, -1, -1, -1],
                ]
            )
        );
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "8".to_string());
    }
}