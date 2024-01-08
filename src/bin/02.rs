use std::ptr::NonNull;

advent_of_code::solution!(2);

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32
}

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Draw>
}

impl Game {
    fn fewest_cubes(&self) -> Draw {
        let mut draw = Draw { red: 0, green: 0, blue: 0 };

        for hand in &self.hands {
            if hand.red > draw.red {
                draw.red = hand.red;
            }
            if hand.green > draw.green {
                draw.green = hand.green;
            }
            if hand.blue > draw.blue {
                draw.blue = hand.blue;
            }
        };

        draw
    }
}

#[derive(Debug)]
struct Games {
    games: Vec<Game>
}

impl Games {
    fn from_input(input: &str) -> Games {
        Games {
            games: input.lines().map(|line| {
                let (game, data) = line.split_once(": ").unwrap();
                let (_, id) = game.split_once(" ").unwrap();

                let draws = data.split("; ").map(|hand| {
                    let mut draw = Draw { red: 0, green: 0, blue: 0 };

                    hand.split(", ").for_each(|color| {
                        let (count, color) = color.split_once(" ").unwrap();

                        match color {
                            "red"   => draw.red += count.parse::<u32>().unwrap(),
                            "green" => draw.green += count.parse::<u32>().unwrap(),
                            "blue"  => draw.blue += count.parse::<u32>().unwrap(),
                            _ => ()
                        }
                    });

                    draw
                }).collect::<Vec<Draw>>();

                Game {
                    id: id.parse::<u32>().unwrap(),
                    hands: draws
                }
            }).collect::<Vec<Game>>()
        }
    }

    fn valid_games(&self, configuration: Draw) -> Vec<&Game> {
        self.games.iter().filter(|game| {
            game.hands.iter().all(|hand| {
                hand.red <= configuration.red &&
                hand.green <= configuration.green &&
                hand.blue <= configuration.blue
            })
        }).collect()
    }


    fn least_cubes(&self) -> Vec<Draw> {
        self.games.iter().map(|game| {
            game.fewest_cubes()
        }).collect::<Vec<Draw>>()
    }

}

pub fn part_one(input: &str) -> Option<u32> {
    let games = Games::from_input(input);
    let valid_games = games.valid_games(Draw { red: 12, green: 13, blue: 14 });
    let ans: u32 = valid_games.iter().map(|g| g.id).sum();

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = Games::from_input(input);
    let draws = games.least_cubes();
    let ans = draws.iter().map(|d| d.red * d.green * d.blue).sum::<u32>();

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
