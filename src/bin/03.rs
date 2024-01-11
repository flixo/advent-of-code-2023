use core::num;
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(3);

#[derive(Debug, Clone, Eq, Hash, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Number {
    value: u32,
    occupation: Vec<Point>
}

impl Number {
    fn neighbouring_coords(&self) -> Vec<Point> {
        self.occupation.iter().flat_map(|point| {
            vec![
                Point { x: point.x + 1, y: point.y },
                Point { x: point.x - 1, y: point.y },
                Point { x: point.x, y: point.y + 1 },
                Point { x: point.x, y: point.y - 1 },

                Point { x: point.x + 1, y: point.y + 1 },
                Point { x: point.x - 1, y: point.y - 1 },
                Point { x: point.x + 1, y: point.y - 1 },
                Point { x: point.x - 1, y: point.y + 1 },
            ]
        })
        .filter(|coord| {
            !self.occupation.contains(coord)
        })
        .unique()
        .collect()
    }
}

#[derive(Debug)]
struct Symbol {
    value: char,
    coord: Point,
    cog_val: Vec<u32>
}

#[derive(Debug)]
struct Schematic {
    width: i32,
    height: i32,
    numbers: Vec<Number>,
    symbols: HashMap<Point, Symbol> ,
}

impl Schematic {
    fn from_input(input: &str) -> Schematic {
        let height = input.lines().count() as i32;
        let width = input.lines().next().unwrap().chars().count() as i32;

        let mut numbers: Vec<Number> = vec![];
        let mut symbols: HashMap<Point, Symbol> = HashMap::new();

        let mut x = 0;
        let mut y = 0;

        let mut number_sequence = String::new();
        let mut occupation: Vec<Point> = vec![];

        input.chars().for_each(|c| {
            let point = Point { x, y };

            if c.is_numeric() {
                number_sequence.push(c);
                occupation.push(point);
            }

            if !c.is_numeric() && number_sequence.len() > 0 {
                let number = number_sequence.parse::<u32>().unwrap();
                
                numbers.push(Number { 
                    value: number, 
                    occupation: occupation.clone()
                });

                occupation.clear();
                number_sequence.clear();
            }

            if !c.is_numeric() && !c.is_ascii_control() && c != '.' {
                symbols.insert(point, Symbol { value: c, coord: point, cog_val: vec![] });
            }

            if c == '\n' {
                x = 0;
                y += 1;
            } else {
                x += 1;
            }
        });

        Schematic { 
            width,
            height,
            numbers,
            symbols
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_input(input);

    let ans: u32 = schematic.numbers.iter().filter(|number| {
        number.neighbouring_coords().iter().any(|coord| {
            schematic.symbols.contains_key(coord)
        })
    }).map(|number| number.value).sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut schematic = Schematic::from_input(input);

    schematic.numbers.iter().for_each(|number| {
        number.neighbouring_coords().iter().for_each(|coord| {
            if let Some(symbol) = schematic.symbols.get_mut(coord) {
                if symbol.value == '*' {
                    symbol.cog_val.push(number.value);
                }
            }
        });
    });

    let ans: u32 = schematic.symbols.values().filter(|symbol| {
        symbol.cog_val.len() == 2
    }).map(|symbol| {
        symbol.cog_val[0] * symbol.cog_val[1]
    }).sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
