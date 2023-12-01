mod day_one;
use day_one::day_one_part_one;

fn load_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

fn main() {
    println!("{}", day_one_part_one().unwrap());
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