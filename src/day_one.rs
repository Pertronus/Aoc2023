use crate::load_file;

pub(crate) fn day_one_part_one() -> Result<u64, std::io::Error> {
    let input = load_file("day_one_input")
        .unwrap_or_else(|_| panic!("Could not load file"));

    let sum: u64 = input.lines().map(|line| {
        let first_digit = line.find(char::is_numeric).unwrap_or_else(|| panic!("Could not find first digit"));
        let frits_digit = line.get(first_digit..first_digit+1).unwrap_or_else(|| panic!("Could not find first digit"));
        let last_digit = line.rfind(char::is_numeric).unwrap_or_else(|| panic!("Could not find last digit"));
        let last_digit = line.get(last_digit..last_digit+1).unwrap_or_else(|| panic!("Could not find last digit"));
        let number= frits_digit.to_string() + last_digit;
        number.parse::<u64>().unwrap_or_else(|_| panic!("Could not parse number"))
    }).sum();
    Ok(sum)
}
