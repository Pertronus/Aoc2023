
#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new() -> Self {
        Self {
            id: 0,
            sets: Vec::new(),
        }
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn add_set(&mut self, set: &str) {
        if set.len() == 0 {
            return;
        }
        set.split(";").for_each(|s| {
            let mut set = Set {
                red: 0,
                green: 0,
                blue: 0,
            };

            s.split(",").for_each(|c| {

                if c.contains("red") {
                    set.red = c.strip_suffix("red").and_then(|c| {
                        Some(c.trim().parse::<u32>().unwrap())
                    }).unwrap_or_else(|| 0);
                } else if c.contains("blue") {
                    set.blue = c.strip_suffix("blue").and_then(|c| {
                        Some(c.trim().parse::<u32>().unwrap())
                    }).unwrap_or_else(|| 0);
                } else if c.contains("green") {
                    set.green = c.strip_suffix("green").and_then(|c| {
                        Some(c.trim().parse::<u32>().unwrap())
                    }).unwrap_or_else(|| 0);
                }
            });

            self.sets.push(set)
        });
    }
}

pub(crate) fn day_two_part_one(input: &str) -> Result<u32, std::io::Error> {

    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let mut game = Game::new();
        line.split(":").for_each(|s| {
            if let Some(game_id) =  s.strip_prefix("Game ") {
                game.set_id(game_id.parse::<u32>().unwrap());
            } else {
                game.add_set(s);
            }
        });
        games.push(game);
    }
    let target_set = Set {
        red: 12,
        green: 13,
        blue: 14
    };


    let mut id_sum = 0;
    for game in &games {
        let mut is_valid = true;
        for set in &game.sets {
            if set.red > target_set.red
                || set.green > target_set.green
                || set.blue > target_set.blue
            {
                is_valid = false;
            }
        }
        if is_valid {
            id_sum += game.id;
        }
    }

    Ok(id_sum)
}


mod test {
    use super::*;

    #[test]
    fn test_day_two_part_one() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let result = day_two_part_one(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }
}

