use std::cmp::{min, max};

advent_of_code::solution!(1);


const PATTERNS: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];


pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().map(|line| {
            let digits: Vec<u32> = line.chars()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .collect();

            digits.first().unwrap() * 10 + digits.last().unwrap()
        }).sum()
    )
}

fn extract_number(string: &str, back: bool) -> Option<u32> {
    for (p, n) in PATTERNS {
        if match back {
            true => string.ends_with(p),
            false => string.starts_with(p),
        } {
            return Some(n)
        }
    }
    return None
}

pub fn part_two(input: &str) -> Option<u32> {
    let ans: u32 = input.lines().map(|line| {
        let mut i = 0;
        
        let mut head: Option<u32> = None;
        let mut tail: Option<u32> = None;

        while head.is_none() || tail.is_none() {
            if head.is_none() {
                head = extract_number(&line[0 + i..line.len()], false);
            }

            if tail.is_none() {
                tail = extract_number(&line[0..line.len() - i], true);
            }

            i +=1;
        }

        if let (Some(head), Some(tail)) = (head, tail) {
            return head * 10 + tail
        } else {
            panic!("No digits found in line: {} \n {:?} - {:?}", line, head, tail);
        }
    }).sum();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
