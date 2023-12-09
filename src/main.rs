mod day_one;
mod day_two;
mod day_three;

use day_one::{ day_one_part_one, day_one_part_two };
use day_two::{ day_two_part_one, day_two_part_two };
use day_three::{ day_three_part_one, day_three_part_two };

fn load_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn main() {
    let input = load_file("day_one_input")
        .unwrap_or_else(|_| panic!("Could not load file"));
    println!("day one part one: {}", day_one_part_one(&input).unwrap());
    println!("day one part two: {}", day_one_part_two(&input).unwrap());

    let input = load_file("day_two_input")
        .unwrap_or_else(|_| panic!("Could not load file"));
    println!("day two part one: {}", day_two_part_one(&input).unwrap());
    println!("day two part two: {}", day_two_part_two(&input).unwrap());

    let input = load_file("day_three_input")
        .unwrap_or_else(|_| panic!("Could not load file"));
    println!("day three part one: {}", day_three_part_one(&input).unwrap());
    println!("day three part two {}", day_three_part_two(&input).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_file() {
        let input = load_file("day_one_input");
        assert!(input.is_ok());
    }
}