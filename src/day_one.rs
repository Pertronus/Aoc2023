
pub(crate) fn day_one_part_one(input: &str) -> Result<u64, std::io::Error> {
    let sum: u64 = input.lines().map(|line| {
        get_calibration_data_v1(line).unwrap()
    }).sum();
    Ok(sum)
}

pub(crate) fn day_one_part_two(input: &str) -> Result<u64, std::io::Error> {
    let sum: u64 = input.lines().map(|line| {
        get_calibration_data_v2(line).unwrap()
    }).sum();
    Ok(sum)
}

fn get_calibration_data_v2(line: &str) -> Result<u64, std::io::Error> {

    let re = regex::Regex::new(r"one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();

    let mut first= re.find(line).unwrap().as_str().to_string();
    let mut second = String::new();

    let mut idx = line.len()-1;
    while idx > 0 {
        if let Some(mat) = re.find_at(line, idx) {
            second = mat.as_str().to_string();
            break;
        }
        idx -= 1;
        continue;
    }

    first = map_number_to_digit(&first);
    second = map_number_to_digit(&second);

    Ok( (first + &second).parse::<u64>().unwrap() )
}

fn map_number_to_digit(number: &str) -> String {
    match number {
        "one" => "1".to_string(),
        "two" => "2".to_string(),
        "three" => "3".to_string(),
        "four" => "4".to_string(),
        "five" => "5".to_string(),
        "six" => "6".to_string(),
        "seven" => "7".to_string(),
        "eight" => "8".to_string(),
        "nine" => "9".to_string(),
        _ => number.to_string()
    }
}

fn get_calibration_data_v1(line: &str) -> Result<u64, std::io::Error> {

    let first = if let Some(digit) = line.find(char::is_numeric).map(|i| {
        line.get(i..i + 1).unwrap()
    }).map(|digit| { digit.to_string() }) {
        digit
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, "Could not find number in line"
        ))
    };

    let second = if let Some(digit) = line.rfind(char::is_numeric).map(|i| {
        line.get(i..i + 1).unwrap()
    }).map(|digit| { digit.to_string() }) {
        digit
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other, "Could not find number in line"
        ))
    };

    Ok( (first + &second).parse::<u64>().unwrap() )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_one_part_one() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".to_string();
        let result = day_one_part_one(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_get_calibration_data_v1() {
        let input = "1abc2".to_string();
        let result = get_calibration_data_v1(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);

        let input = "pqr3stu8vwx".to_string();
        let result = get_calibration_data_v1(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 38);
    }

    #[test]
    fn test_get_calibration_data_v2() {
        let input = "1abc2".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);

        let input = "pqr3stu8vwx".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 38);

        let input = "asdfoneight".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 18);

        let input = "two1nine".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 29);

        let input = "eightwothree".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 83);

        let input = "abcone2threexyz".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13);

        let input = "xtwone3four".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 24);

        let input = "4nineeightseven2".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);

        let input = "zoneight234".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 14);

        let input = "7pqrstsixteen".to_string();
        let result = get_calibration_data_v2(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 76);
    }

    #[test]
    fn test_day_one_part_two() {
        let input = "two1nine\neightwothree\nbcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen".to_string(); let result = day_one_part_two(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 281);
    }
}
