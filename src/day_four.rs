
pub(crate) fn day_four_part_one(input: &str) -> u32 {
    let mut total_score = 0;
    for game_string in input.lines() {
        let game_card_tuple = split_game_card(game_string);
        total_score += score_game_card(game_card_tuple);
    }
    total_score
}

fn get_card_numbers(card_number_string: &str) -> Vec<u32> {
    card_number_string.split(" ").filter_map(|x| {
        if x.is_empty() {
            None
        } else {
            Some(x.parse::<u32>().unwrap())
        }
    }).collect()
}

fn split_game_card(game_card_string: &str) -> (u32, Vec<u32>, Vec<u32>) {

    let mut game_tuple : (u32, Vec<u32>, Vec<u32>) = (0, Vec::new(), Vec::new());

    let card_data: Vec<&str> = game_card_string.split(":").collect();
    game_tuple.0 = card_data[0].strip_prefix("Card ").unwrap().trim().parse::<u32>().unwrap();
    let game_numbers = card_data[1].split("|").collect::<Vec<&str>>();
    game_tuple.1 = get_card_numbers(game_numbers[0]);
    game_tuple.2 = get_card_numbers(game_numbers[1]);
    game_tuple
}

fn score_game_card(game_card_tuple: (u32, Vec<u32>, Vec<u32>)) -> u32 {

    let (_, card_numbers, winning_numbers) = game_card_tuple;

    let matches = card_numbers.iter().filter_map(|n| {
        if winning_numbers.contains(n) {
            Some(n)
        } else {
            None
        }
    }).collect::<Vec<&u32>>().len() as u32;

    if matches == 0 {
       0
    } else {
       1 << (matches - 1)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_get_winning_numbers() {
        let winning_number_string = "41 48 83 86 17";
        let winning_number_vec : Vec<u32> = vec![41, 48, 83, 86, 17];
        assert_eq!(get_card_numbers(winning_number_string), winning_number_vec);
    }

    #[test]
    fn test_get_card_numbers() {
        let card_number_string = "83 86  6 31 17  9 48 53";
        let card_number_vec : Vec<u32> = vec![83, 86,  6, 31, 17, 9, 48, 53];
        assert_eq!(get_card_numbers(card_number_string), card_number_vec);
    }

    #[test]
    fn test_split_game_card() {
        let game_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_card_tuple = (1, vec![41, 48, 83, 86, 17], vec![83, 86,  6, 31, 17, 9, 48, 53]);
        assert_eq!(split_game_card(game_string), game_card_tuple);
    }

    #[test]
    fn test_score_game_card() {
        let game_card_tuple = (1, vec![41, 48, 83, 86, 17], vec![83, 86,  6, 31, 17, 9, 48, 53]);
        assert_eq!(score_game_card(game_card_tuple), 8);
    }

    #[test]
    fn test_game_parse() {
        let game_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let game_card_tuple = split_game_card(game_string);
        assert_eq!(score_game_card(game_card_tuple), 8);
    }

    #[test]
    fn test_day_four_part_one() {
        let input = indoc! {"
                    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
                    "};

        assert_eq!(day_four_part_one(input), 13);
    }
}