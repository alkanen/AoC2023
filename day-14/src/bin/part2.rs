use indicatif::ProgressBar;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Round,
    Square,
    Empty,
}
impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            'O' => Tile::Round,
            '#' => Tile::Square,
            '.' => Tile::Empty,
            _ => panic!("Unknown tile: {}", c),
        }
    }
    fn to_char(&self) -> char {
        match self {
            Tile::Round => 'O',
            Tile::Square => '#',
            Tile::Empty => '.',
        }
    }
}
pub struct Board {
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    // A method to turn the board into a string.
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in self.tiles.iter() {
            for tile in row.iter() {
                result.push(tile.to_char());
            }
            result.push('\n');
        }
        return result;
    }

    fn roll_north(&mut self) {
        for row in 1..self.tiles.len() {
            for col in 0..self.tiles[row].len() {
                let tile = self.tiles[row][col];

                if tile == Tile::Round {
                    // Check the tile above.
                    let mut replace_row = row;

                    for row_above in (0..row).rev() {
                        let tile_above = self.tiles[row_above][col];
                        if tile_above == Tile::Empty {
                            replace_row = row_above;
                        } else {
                            break;
                        }
                    }

                    // Move the tile above down.
                    self.tiles[row][col] = Tile::Empty;
                    self.tiles[replace_row][col] = Tile::Round;
                }
            }
        }
    }

    fn roll_south(&mut self) {
        for row in (0..self.tiles.len() - 1).rev() {
            for col in 0..self.tiles[row].len() {
                let tile = self.tiles[row][col];

                if tile == Tile::Round {
                    // Check the tile above.
                    let mut replace_row = row;

                    for row_above in (row + 1)..self.tiles.len() {
                        let tile_above = self.tiles[row_above][col];
                        if tile_above == Tile::Empty {
                            replace_row = row_above;
                        } else {
                            break;
                        }
                    }

                    // Move the tile above down.
                    self.tiles[row][col] = Tile::Empty;
                    self.tiles[replace_row][col] = Tile::Round;
                }
            }
        }
    }

    fn roll_west(&mut self) {
        for col in 1..self.tiles[0].len() {
            for row in 0..self.tiles.len() {
                let tile = self.tiles[row][col];

                if tile == Tile::Round {
                    // Check the tile above.
                    let mut replace_col = col;

                    for col_left in (0..col).rev() {
                        let tile_left = self.tiles[row][col_left];
                        if tile_left == Tile::Empty {
                            replace_col = col_left;
                        } else {
                            break;
                        }
                    }

                    // Move the tile above down.
                    self.tiles[row][col] = Tile::Empty;
                    self.tiles[row][replace_col] = Tile::Round;
                }
            }
        }
    }

    fn roll_east(&mut self) {
        for col in (0..self.tiles[0].len() - 1).rev() {
            for row in 0..self.tiles.len() {
                let tile = self.tiles[row][col];

                if tile == Tile::Round {
                    // Check the tile above.
                    let mut replace_col = col;

                    for col_left in (col + 1)..self.tiles[0].len() {
                        let tile_left = self.tiles[row][col_left];
                        if tile_left == Tile::Empty {
                            replace_col = col_left;
                        } else {
                            break;
                        }
                    }

                    // Move the tile above down.
                    self.tiles[row][col] = Tile::Empty;
                    self.tiles[row][replace_col] = Tile::Round;
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    fn weight(&self) -> i64 {
        let mut result = 0;
        for (i, row) in self.tiles.iter().enumerate() {
            for tile in row.iter() {
                if *tile == Tile::Round {
                    result += self.tiles.len() as i64 - i as i64;
                }
            }
        }
        return result;
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut board = parse(input);

    let num_cycles: u64 = 1000000000;

    let bar = ProgressBar::new(num_cycles);
    for i in 0..num_cycles {
        board.cycle();
        bar.inc(1);

        /*
        if i < 20 {
            println!("i: ({})\n{}\n\n=====================", board.weight(), board.to_string().trim());
        } else {
            panic!();
        }
        */
        if i < 500 {
            print!("{}, ", board.weight());
            if (i + 1) % 9 == 0 {
                println!();
            }
        } else {
            panic!();
        }
    }
    bar.finish();

    return board.weight().to_string();
}

fn parse(input: &str) -> Board {
    let mut tiles = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(Tile::from_char(c));
        }
        tiles.push(row);
    }
    return Board { tiles };
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_parse() {
        let board = parse(INPUT);
        assert_eq!(board.to_string().trim(), INPUT.trim());
    }


    #[test]
    fn test_roll_north() {
        let mut board = parse(INPUT);
        board.roll_north();

        assert_eq!(
            board.to_string().trim(),
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
        );
    }

    #[test]
    fn test_roll_south() {
        let mut board = parse(INPUT);
        board.roll_south();


        println!("{}", board.to_string().trim());

        assert_eq!(
            board.to_string().trim(),
            ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"
        );
    }

    #[test]
    fn test_roll_west() {
        let mut board = parse(INPUT);
        board.roll_west();


        println!("{}", board.to_string().trim());

        assert_eq!(
            board.to_string().trim(),
            "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."
        );
    }

    #[test]
    fn test_roll_east() {
        let mut board = parse(INPUT);
        board.roll_east();


        println!("{}", board.to_string().trim());

        assert_eq!(
            board.to_string().trim(),
            "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."
        );
    }

    #[test]
    fn test_one_cycle() {
        let mut board = parse(INPUT);
        board.cycle();


        println!("{}", board.to_string().trim());

        assert_eq!(
            board.to_string().trim(),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );
    }
    
    #[test]
    fn test_weight() {
        let mut board = parse(INPUT);

        board.roll_north();
        assert_eq!(board.weight(), 136);
    }


    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "64".to_string());
    }
}