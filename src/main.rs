mod day_one;
use day_one::day_one_part_one;
use crate::day_one::day_one_part_two;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn main() {
    let input = load_file("day_one_input")
        .unwrap_or_else(|_| panic!("Could not load file"));
    println!("{}", day_one_part_one(&input).unwrap());
    println!("{}", day_one_part_two(&input).unwrap());

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