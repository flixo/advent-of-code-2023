use std::collections::{VecDeque, HashMap};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let ans: u32 = input.lines().map(|line| {
        let mut data = line.split_whitespace().skip(2).map(|s| s.parse().unwrap_or_default());

        let winners: Vec<u32> = data.by_ref().take_while(|x| x != &0).collect();
        let numbers: Vec<u32> = data.by_ref().take_while(|x| x != &0).collect();

        numbers.iter().fold(0, |acc, x| {
            match (winners.contains(&x), acc == 0) {
                (true, true)  => 1,
                (true, false) => acc * 2,
                (_, _)        => acc
            }
        })
    }).sum();

    Some(ans)
}


struct Card {
    numbers: Vec<u32>,
    winners: Vec<u32>,
    index: usize,
}

impl Card {
    fn wins(&self) -> usize {
        self.numbers.iter().fold(0, |acc, x| {
            match (self.winners.contains(&x), acc == 0) {
                (true, true)  => 1,
                (true, false) => acc * 2,
                (_, _)        => acc
            }
        })
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut index: usize = 0;
    let cards: HashMap<usize, Card> = input.lines().map(|line| {
        let mut data = line.split_whitespace().skip(2).map(|s| s.parse::<u32>().unwrap_or_default());

        let winners: Vec<u32> = data.by_ref().take_while(|x| x != &0).collect();
        let numbers: Vec<u32> = data.by_ref().take_while(|x| x != &0).collect();

        let c = Card {index, winners, numbers};
        index += 1;
        c
    }).enumerate().collect();

    let mut to_process: VecDeque<usize> = cards.keys().map(|k| *k).collect();
    let mut processed_count: u32 = 0;

    loop {
        match to_process.pop_front() {
            Some(key) => {
                let card = cards.get(&key).expect("Missing Card");

                for i in 0..card.wins() {
                    match cards.get(&(card.index + 1 + i)) {
                        Some(c) => to_process.push_back(c.index),
                        None => continue,
                    }
                }
                processed_count += 1;
            },

            None => break,
        }
    }

    Some(processed_count - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
