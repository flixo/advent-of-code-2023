advent_of_code::solution!(3);

#[derive(Debug)]
struct Schematic {
    size: (usize, usize),
    data: Vec<char>,
    to_inspect: Vec<usize>
}

impl Schematic {
    fn from_input(input: &str) -> Schematic {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        let mut to_inspect: Vec<usize> = vec![];
        let mut data: Vec<char> = vec![];

        input.chars().filter(|c| !c.is_whitespace()).enumerate().for_each(|(i, c)| {
            if c.is_numeric() {
                to_inspect.push(i);
            }
            if !c.is_whitespace() {
                data.push(c);
            }
        });

        Schematic { 
            size: (width, height),
            data: data,
            to_inspect: to_inspect
        }
    }

    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        let (width, _) = self.size;
        let x = idx % width;
        let y = idx / width;

        (x, y)
    }

    fn coord_to_idx(&self, coord: (usize, usize)) -> usize {
        let (width, _) = self.size;
        let (x, y) = coord;

        y * width + x
    }
    
    fn data_at(&self, coord: (usize, usize)) -> char {
        let idx = self.coord_to_idx(coord);
        self.data[idx]
    }

    fn data_at_idx(&self, idx: usize) -> char {
        self.data[idx]
    }
}



pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_input(input);

    schematic.to_inspect.iter().for_each(|idx| {
        let coord = schematic.idx_to_coord(*idx);

        //vec![(1,0),()]

        println!("{:?}", coord);
    });

    //println!("{:?}", schematic.data_at((2, 2)));


    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
