use std::collections::HashMap;

pub(crate) fn day_three_part_one(input: &str) -> Result<u32, std::io::Error> {

    let mut part_sum = 0;
    for y in 0..input.lines().count() {
        let char_count = input.lines().nth(y).unwrap().chars().count();
        let mut x = 0;
        while x < char_count {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if char.is_numeric()  {
                if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, x, y) {
                    if is_part(input, x1, y1, x2, y2) {
                        part_sum += part;
                    }
                    if x2 + 1 >= char_count {
                        break;
                    }
                    x = x2;
                    continue;
                }
            }
            x += 1;
        }
    }

    if part_sum == 0 {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "No part found"))
    } else {
        Ok(part_sum)
    }
}

pub(crate) fn day_three_part_two(input: &str) -> Result<u32, std::io::Error> {
    let mut calibration_sum = 0;
    let mut potential_gear_map: HashMap<(usize, usize), u32> = HashMap::new();
    for y in 0..input.lines().count() {
        let char_count = input.lines().nth(y).unwrap().chars().count();
        let mut x = 0;
        while x < char_count {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if char.is_numeric()  {
                if let Some((gear, x1, y1, x2, y2)) = get_boundary_coordinates(input, x, y) {
                    if let Some((x1, y1)) = is_potential_gear(input, x1, y1, x2, y2) {
                        if !potential_gear_map.contains_key(&(x1, y1)) {
                            potential_gear_map.insert((x1, y1), gear);
                        } else {
                            calibration_sum += gear * potential_gear_map.get(&(x1, y1)).unwrap();
                        }
                    }
                    if x2 + 1 >= char_count {
                        break;
                    }
                    x = x2;
                    continue;
                }
            }
            x += 1;
        }
    }

    if calibration_sum == 0 {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "No part found"))
    } else {
        Ok(calibration_sum)
    }
}

fn get_boundary_coordinates(input: &str, x_offset: usize, y_offset: usize) -> Option<(u32,usize, usize, usize, usize)> {
    let line_count = input.lines().count();
    for y in y_offset..line_count {
        let mut boundary: (u32, usize, usize, usize, usize) = (0, usize::MAX, usize::MAX, usize::MAX, usize::MAX);
        let char_count = input.lines().nth(y).unwrap().chars().count();
        for x in x_offset..char_count {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if char.is_numeric()  {
                boundary.0 = format!("{}{}", boundary.0, char).parse::<u32>().unwrap();
                if boundary.1 == usize::MAX {
                    boundary.1 = if x == 0 { x } else { x - 1 };
                    boundary.2 = if y == 0 { y } else { y - 1 };
                }
                if x + 1 == char_count {
                    boundary.3 = x;
                    boundary.4 = if y + 1 == line_count { y } else { y + 1 };
                    return Some(boundary);
                }
            } else if boundary.1 != usize::MAX && boundary.3 == usize::MAX {
                boundary.3 = x;
                boundary.4 = if y + 1 == line_count { y } else { y + 1 };
                return Some(boundary);
            }
        }
    }
    None
}

fn is_part(input: &str, x1: usize, y1: usize, x2: usize, y2: usize) -> bool {
    for y in y1..=y2 {
        for x in x1..=x2 {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if !char.is_numeric() && char != '.' {
                return true;
            }
        }
    }
    false
}

fn is_potential_gear(input: &str, x1: usize, y1: usize, x2: usize, y2: usize) -> Option<(usize, usize)> {
    for y in y1..=y2 {
        for x in x1..=x2 {
            let char = input.lines().nth(y).unwrap().chars().nth(x).unwrap();
            if char == '*' {
                return Some((x, y));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_is_potential_gear() {
        let input = indoc! {"
                .......
                .10*...
                .......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_potential_gear(input, x1, y1, x2, y2).is_some());
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_fetch_number_and_adjacent_grid() {
        let input = indoc! {"
                ......
                ......
                ..10..
                ......
                ......
                "};
        let result = get_boundary_coordinates(input, 0, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 1, 1, 4, 3));
    }

    #[test]
    fn test_find_part_1() {
        let input = indoc! {"
                ......
                .+....
                ..10..
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_2() {
        let input = indoc! {"
                ......
                ......
                ..10..
                ....+.
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_find_part_3() {
        let input = indoc! {"
                ......
                ......
                ..10..
                .+....
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_4() {
        let input = indoc! {"
                ......
                ....+.
                ..10..
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_fetch_number_with_offset_after_number() {
        let input = indoc! {"
                ......
                ......
                ..10..
                ......
                ......
                "};
        let result = get_boundary_coordinates(input, 4, 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_fetch_number_with_offset_before_number() {
        let input = indoc! {"
                ......
                ......
                ..10..
                ......
                ......
                "};
        let result = get_boundary_coordinates(input, 1, 2);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 1, 1, 4, 3));
    }

    #[test]
    fn test_fetch_number_top_left() {
        let input = indoc! {"
                10....
                ......
                ......
                ......
                ......
                "};
        let result = get_boundary_coordinates(input, 0, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 0, 0, 2, 1));
    }

    #[test]
    fn test_find_part_5() {
        let input = indoc! {"
                10+...
                ......
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_6() {
        let input = indoc! {"
                10....
                ..+...
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_find_part_7() {
        let input = indoc! {"
                10....
                +.....
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_fetch_number_top_right() {
        let input = indoc! {"
                ....10
                ......
                ......
                ......
                ......
                "};
        let result = get_boundary_coordinates(input, 0, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 3, 0, 5, 1));
    }

    #[test]
    fn test_find_part_8() {
        let input = indoc! {"
                ....10
                .....+
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_9() {
        let input = indoc! {"
                ....10
                ...+..
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_find_part_10() {
        let input = indoc! {"
                ...+10
                ......
                ......
                ......
                ......
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_fetch_number_bottom_left() {
        let input = indoc! {"
                ......
                ......
                ......
                ......
                10....
                "};
        let result = get_boundary_coordinates(input, 0, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 0, 3, 2, 4));
    }

    #[test]
    fn test_find_part_11() {
        let input = indoc! {"
                ......
                ......
                ......
                +.....
                10....
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_12() {
        let input = indoc! {"
                ......
                ......
                ......
                ..+...
                10....
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_find_part_13() {
        let input = indoc! {"
                ......
                ......
                ......
                ......
                10+...
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_fetch_number_bottom_right() {
        let input = indoc! {"
                ......
                ......
                ......
                ......
                ....10
                "};
        let result = get_boundary_coordinates(input, 0, 0);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (10, 3, 3, 5, 4));
    }

    #[test]
    fn test_find_part_14() {
        let input = indoc! {"
                ......
                ......
                ......
                .....+
                ....10
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }

    #[test]
    fn test_find_part_15() {
        let input = indoc! {"
                ......
                ......
                ......
                ...+..
                ....10
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_find_part_16() {
        let input = indoc! {"
                ......
                ......
                ......
                ......
                ...+10
                "};
        if let Some((part, x1, y1, x2, y2)) = get_boundary_coordinates(input, 0, 0) {
            assert!(is_part(input, x1, y1, x2, y2));
            assert_eq!(part, 10);
        }
    }
    #[test]
    fn test_day_three_part_one_part_at_sample() {
        let input = indoc! {"
                    467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..
                    "};
        let result = day_three_part_one(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4361);
    }

    #[test]
    fn test_day_three_part_two_sample() {
        let input = indoc! {"
                    467..114..
                    ...*......
                    ..35..633.
                    ......#...
                    617*......
                    .....+.58.
                    ..592.....
                    ......755.
                    ...$.*....
                    .664.598..
                    "};
        let result = day_three_part_two(&input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 467835);
    }
}