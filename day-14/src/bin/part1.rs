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
    board.roll_north();
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
    fn test_weight() {
        let mut board = parse(INPUT);
        board.roll_north();
        assert_eq!(board.weight(), 136);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "136".to_string());
    }
}