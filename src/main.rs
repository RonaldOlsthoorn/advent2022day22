
use std::{io::{BufReader, BufRead}, fs::File};

const BACKSPACE: char = 8u8 as char;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ControlInput {
    RotateRight,
    RotateLeft,
    Move(usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up, Down, Left, Right
}

impl Direction {

    fn rotate_right(&mut self) {
        match self {
            Self::Up => {*self = Self::Right;},
            Self::Right => {*self = Self::Down;},
            Self::Down => {*self = Self::Left;},
            Self::Left => {*self = Self::Up;}           
        }
    }

    fn rotate_left(&mut self) {
        match self {
            Self::Up => {*self = Self::Left;},
            Self::Left => {*self = Self::Down;},
            Self::Down => {*self = Self::Right;},
            Self::Right => {*self = Self::Up;}           
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ChamberOccupation {
    Void,
    Wall,
    Open
}

fn simulate(chamber : Vec<Vec<ChamberOccupation>>, inputs: Vec<ControlInput>) -> (usize, usize, Direction) {

    let mut position = (0, chamber[0].iter().position(|x| *x != ChamberOccupation::Void).unwrap());
    let mut direction = Direction::Right;

    let chamber_width = chamber[0].len();
    let chamber_height = chamber.len();

    for (iteration, input) in inputs.into_iter().enumerate() {

        match input {
            ControlInput::RotateLeft => direction.rotate_left(),
            ControlInput::RotateRight => direction.rotate_right(),
            ControlInput::Move(tiles) => {

                let mut new_row = position.0;
                let mut new_col = position.1;

                for _ in 0..tiles {

                    match direction {
                        Direction::Up => {
                            if new_row == 0 || chamber[new_row - 1][new_col] == ChamberOccupation::Void {
                                let (new_row_candidate, new_col_candidate, new_dir_candidate) 
                                    = wrap_candidate(&new_row, &new_col, &direction);
                                if chamber[new_row_candidate][new_col_candidate] == ChamberOccupation::Wall {
                                    break;
                                }
                                new_row = new_row_candidate;
                                new_col = new_col_candidate;
                                direction = new_dir_candidate;
                            } else if chamber[new_row - 1][new_col] == ChamberOccupation::Wall {
                                break;
                            } else {
                                new_row -= 1;
                            }
                        },
                        Direction::Down => {
                            if new_row + 1 == chamber_height || chamber[new_row + 1][new_col] == ChamberOccupation::Void {
                                let (new_row_candidate, new_col_candidate, new_dir_candidate) 
                                    = wrap_candidate(&new_row, &new_col, &direction);
                                if chamber[new_row_candidate][new_col_candidate] == ChamberOccupation::Wall {
                                    break;
                                }
                                new_row = new_row_candidate;
                                new_col = new_col_candidate;
                                direction = new_dir_candidate;
                            } else if chamber[new_row + 1][new_col] == ChamberOccupation::Wall {
                                break;
                            } else {
                                new_row += 1;
                            }
                        },
                        Direction::Right => {
                            if new_col + 1 == chamber_width || chamber[new_row][new_col + 1] == ChamberOccupation::Void {
                                let (new_row_candidate, new_col_candidate, new_dir_candidate) 
                                    = wrap_candidate(&new_row, &new_col, &direction);
                                if chamber[new_row_candidate][new_col_candidate] == ChamberOccupation::Wall {
                                    break;
                                }
                                new_row = new_row_candidate;
                                new_col = new_col_candidate;
                                direction = new_dir_candidate;
                            } else if chamber[new_row][new_col + 1] == ChamberOccupation::Wall {
                                break;
                            } else {
                                new_col += 1;
                            }
                        },
                        Direction::Left => {
                            if new_col == 0 || chamber[new_row][new_col - 1] == ChamberOccupation::Void {
                                let (new_row_candidate, new_col_candidate, new_dir_candidate) 
                                    = wrap_candidate(&new_row, &new_col, &direction);
                                if chamber[new_row_candidate][new_col_candidate] == ChamberOccupation::Wall {
                                    break;
                                }
                                new_row = new_row_candidate;
                                new_col = new_col_candidate;
                                direction = new_dir_candidate;
                            } else if chamber[new_row][new_col - 1] == ChamberOccupation::Wall {
                                break;
                            } else {
                                new_col -= 1;
                            }
                        }
                    }
                }

                position = (new_row, new_col);
            }
        }

        println!("row: {}, column {}, direction {:?}", position.0 + 1, position.1 + 1, direction);
    }
    
    (position.0, position.1, direction)
}

fn wrap_candidate(row: &usize, col: &usize, dir: &Direction) -> (usize, usize, Direction) {

    if row >= &0 && row <= &49 && col == &50 && dir == &Direction::Left {           // A left to E left
       return  (149 - row, 0, Direction::Right);
    } else if row >= &50 && row <= &99 && col == &50 && dir == &Direction::Left {   // C left to E top
        return (100, row - 50, Direction::Down);
    } else if row == &100 && col >= &0 && col <= &49 && dir == &Direction::Up {     // E top to C left
        return (col + 50, 50, Direction::Right);
    } else if row >= &100 && row <= &149 && col == &0 && dir == &Direction::Left {  // E left to A left
        return (149 - row, 50, Direction::Right);
    } else if row >= &150 && row <= &199 && col == &0 && dir == &Direction::Left {  // F left to A top
        return (0, row - 100, Direction::Down);
    } else if row == &199 && col >= &0 && col <= &49 && dir == &Direction::Down {   // F bottom to B top
        return (0, col + 100, Direction::Down);
    } else if row >= &150 && row <= &199 && col == &49 && dir == &Direction::Right {// F right to D bottom
        return (149, row - 100, Direction::Up);
    } else if row == &149 && col >= &50 && col <= &99 && dir == &Direction::Down {  // D bottom to F right
        return (col + 100, 49, Direction::Left);
    } else if row >= &100 && row <= &149 && col == &99 && dir == &Direction::Right {// D right to B right
        return (149 - row, 149, Direction::Left);
    } else if row >= &50 && row <= &99 && col == &99 && dir == &Direction::Right {  // C right to B bottom
        return (49, 50 + row, Direction::Up);
    } else if row == &49 && col >= &100 && col <= &149 && dir == &Direction::Down { // B bottom to C right
        return (col - 50, 99, Direction::Left);
    } else if row >= &0 && row <= &50 && col == &149 && dir == &Direction::Right{   // B right to D right
        return (149 - row, 99, Direction::Left);
    } else if row == &0 && col >= &100 && col <= &149 && dir == &Direction::Up {    // B top to F bottom
        return (199, col - 100, Direction::Up);
    } else if row == &0 && col >= &50 && col <= &99 && dir == &Direction::Up{       // A top to F left
        return (col + 100, 0, Direction::Right);
    }

    panic!("wrap ran out of options row {}, col {}, dir {:?}", row, col, dir);

}

fn print(chamber: &Vec<Vec<ChamberOccupation>>, position: &(usize, usize), direction: &Direction) {

    for (row_index, row) in chamber.iter().enumerate() {

        let mut print_row = String::new();
        for (column_index, tile) in row.iter().enumerate() {
            if row_index == position.0 && column_index == position.1 {
                match direction {
                    Direction::Up => {print_row.push('^');},
                    Direction::Left => {print_row.push('<');},
                    Direction::Right => {print_row.push('>');},
                    Direction::Down => {print_row.push('v');}
                }
            } else {
                match tile {
                    ChamberOccupation::Void => {print_row.push(' ');},
                    ChamberOccupation::Wall => {print_row.push('#');},
                    ChamberOccupation::Open => {print_row.push('.');}
                }
            }
        }
        println!("{}", print_row);
    }
    
}


fn main() {

    let lines: Vec<String> = BufReader::new(File::open("input.txt").unwrap()).lines().map(|l| l.unwrap()).collect();

    let chamber_height = lines.len() - 2;
    let chamber_width = lines[0..chamber_height].iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len();

    let mut chamber : Vec<Vec<ChamberOccupation>> = Vec::with_capacity(chamber_height);

    for row in &lines[0..chamber_height] {

        let mut tiles = vec![ChamberOccupation::Void; chamber_width];

        row.chars().enumerate().for_each(|(i, c)| 
            if c == ' ' {tiles[i] = ChamberOccupation::Void}
            else if c=='.' {tiles[i] = ChamberOccupation::Open}
            else {tiles[i] = ChamberOccupation::Wall});
        
        chamber.push(tiles);
    }

    let mut inputs: Vec<ControlInput> = Vec::new();
    let mut cache: Vec<char> = Vec::new();

    for c in lines.iter().last().unwrap().chars() {
        if c == 'R' {
            if !cache.is_empty() {
                let s: String = cache.iter().collect();
                inputs.push(ControlInput::Move(s.parse::<usize>().unwrap()));
                cache.clear();
            }
            inputs.push(ControlInput::RotateRight);
        } else if c == 'L' {
            if !cache.is_empty() {
                let s: String = cache.iter().collect();
                inputs.push(ControlInput::Move(s.parse::<usize>().unwrap()));
                cache.clear();
            }
            inputs.push(ControlInput::RotateLeft);
        } else {
            cache.push(c);
        }
    }

    if !cache.is_empty() {
        let s: String = cache.iter().collect();
        inputs.push(ControlInput::Move(s.parse::<usize>().unwrap()));
        cache.clear();
    }

    let end_position = simulate(chamber, inputs);

    println!("end state {0}, {1}, {2:?}", end_position.0, end_position.1, end_position.2);

    let mut res = 1000 * (end_position.0 + 1) + 4 * (end_position.1 + 1);

    match end_position.2 {
        Direction::Right => {},
        Direction::Down => {res += 1;},
        Direction::Left => {res += 2;},
        Direction::Up => {res += 3;},
    }

    println!("Final result: {}", res);
}