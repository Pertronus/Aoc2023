
#[derive(PartialEq, Debug)]
struct Map {
    source: u64,
    dest: u64,
    range: u64
}

#[derive(PartialEq, Debug)]
struct SeedLocation {
    seeds: Vec<u64>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water:Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}


impl SeedLocation {
    fn new() -> SeedLocation {
        SeedLocation {
            seeds: Vec::new(),
            seed_to_soil: Vec::new(),
            soil_to_fertilizer: Vec::new(),
            fertilizer_to_water: Vec::new(),
            water_to_light: Vec::new(),
            light_to_temperature: Vec::new(),
            temperature_to_humidity: Vec::new(),
            humidity_to_location: Vec::new(),
        }
    }

    fn parse_input(&mut self, input: &str) {
        let mut lines = input.lines();
        let mut map_to_fill = &mut self.seed_to_soil;
        while let Some(line) = lines.next() {
            if line.is_empty() {
                continue;
            }
            if line.contains(":") {
                if line.contains("seeds") {
                    line.strip_prefix("seeds:").unwrap().trim().split(" ").map(|x| x.parse::<u64>().unwrap()).for_each(|x| self.seeds.push(x));
                } else if line.contains("seed-to-soil") {
                    map_to_fill = &mut self.seed_to_soil;
                } else if line.contains("soil-to-fertilizer") {
                    map_to_fill = &mut self.soil_to_fertilizer;
                } else if line.contains("fertilizer-to-water") {
                    map_to_fill = &mut self.fertilizer_to_water;
                } else if line.contains("water-to-light") {
                    map_to_fill = &mut self.water_to_light;
                } else if line.contains("light-to-temperature") {
                    map_to_fill = &mut self.light_to_temperature;
                } else if line.contains("temperature-to-humidity") {
                    map_to_fill = &mut self.temperature_to_humidity;
                } else if line.contains("humidity-to-location") {
                    map_to_fill = &mut self.humidity_to_location;
                } else {
                    panic!("Unknown map type: {}", line);
                }
                continue;
            }
            let mut map_iter = line.split_whitespace();
            let dest = map_iter.next().unwrap().parse::<u64>().unwrap();
            let src = map_iter.next().unwrap().parse::<u64>().unwrap();
            let range = map_iter.next().unwrap().parse::<u64>().unwrap();
            map_to_fill.push(Map { source: src, dest, range });
        }
    }

    fn map_src_to_dest(&self, input: u64, map: &Vec<Map>) -> u64 {
        for m in map {
            if input >= m.source && input <= m.source + m.range {
                return m.dest + input - m.source;
            }
        }
        input
    }

    fn seed_to_location(&self, input: u64) -> u64 {

        let mut seed = input;

        seed = self.map_src_to_dest(seed, &self.seed_to_soil);
        seed = self.map_src_to_dest(seed, &self.soil_to_fertilizer);
        seed = self.map_src_to_dest(seed, &self.fertilizer_to_water);
        seed = self.map_src_to_dest(seed, &self.water_to_light);
        seed = self.map_src_to_dest(seed, &self.light_to_temperature);
        seed = self.map_src_to_dest(seed, &self.temperature_to_humidity);
        seed = self.map_src_to_dest(seed, &self.humidity_to_location);

        seed

    }

    fn get_min_location(&self) -> u64 {
        self.seeds.iter().map(|x| self.seed_to_location(*x)).min().unwrap()
    }
}

pub(crate) fn day_five_part_one(input: &str) -> u64 {
    let mut seed_location = SeedLocation::new();
    seed_location.parse_input(input);
    seed_location.get_min_location()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn map_src_to_dest() {
        let seed_location = SeedLocation::new();
        let map = vec![
            Map { source: 98, dest: 50, range: 2 },
            Map { source: 50, dest: 52, range: 98 }
        ];
        assert_eq!(seed_location.map_src_to_dest(98, &map), 50);
        assert_eq!(seed_location.map_src_to_dest(53, &map), 55);
        assert_eq!(seed_location.map_src_to_dest(10, &map), 10);
    }

    #[test]
    fn parse_min_location() {
        let mut seed_location = SeedLocation::new();
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "};


        seed_location.parse_input(input);
        assert_eq!(seed_location.get_min_location(), 35);
    }

    #[test]
    fn parse_seeds() {
        let mut seed_location = SeedLocation::new();
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "};

        let seed = 79;

        seed_location.parse_input(input);
        assert_eq!(seed_location.seed_to_location(seed), 82);
    }

    #[test]
    fn parse_seed_to_soil() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            seed-to-soil map:
            50 98 2
            52 50 48
        "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.seed_to_soil, vec![Map { dest: 50, source: 98, range: 2 }, Map { dest: 52, source: 50, range: 48 }]);
    }

    #[test]
    fn parse_soil_to_fertilizer() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.soil_to_fertilizer,
                   vec![Map { dest: 0, source: 15, range: 37 },
                        Map { dest: 37, source: 52, range: 2 },
                        Map { dest: 39, source: 0, range: 15 }]);
    }

    #[test]
    fn parse_fertilizer_to_water() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.fertilizer_to_water,
                   vec![Map { dest: 49, source: 53, range: 8 },
                        Map { dest: 0, source: 11, range: 42 },
                        Map { dest: 42, source: 0, range: 7 },
                        Map { dest: 57, source: 7, range: 4 }]);
    }

    #[test]
    fn parse_water_to_light() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            water-to-light map:
            88 18 7
            18 25 70
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.water_to_light,
                   vec![Map { dest: 88, source: 18, range: 7 },
                        Map { dest: 18, source: 25, range: 70 }]);
    }

    #[test]
    fn parse_light_to_temperature() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.light_to_temperature,
                   vec![Map { dest: 45, source: 77, range: 23 },
                        Map { dest: 81, source: 45, range: 19 },
                        Map { dest: 68, source: 64, range: 13 }]);
    }

    #[test]
    fn parse_temperature_to_humidity() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            temperature-to-humidity map:
            0 69 1
            1 0 69
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.temperature_to_humidity,
                   vec![Map { dest: 0, source: 69, range: 1 },
                        Map { dest: 1, source: 0, range: 69 }]);
    }

    #[test]
    fn parse_humidity_to_location() {
        let mut seed_location = SeedLocation::new();

        let input = indoc! {"
            humidity-to-location map:
            0 69 1
            1 0 69
            "};

        seed_location.parse_input(input);
        assert_eq!(seed_location.humidity_to_location,
                   vec![Map { dest: 0, source: 69, range: 1 },
                        Map { dest: 1, source: 0, range: 69 }]);
    }

    #[test]
    fn parse_input() {
        let mut seed_location = SeedLocation::new();
        let input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "};


        seed_location.parse_input(input);
        assert_eq!(seed_location, SeedLocation {
            seeds: vec![79, 14, 55, 13],
            seed_to_soil: vec![Map { dest: 50, source: 98, range: 2 }, Map { dest: 52, source: 50, range: 48 }],
            soil_to_fertilizer: vec![Map { dest: 0, source: 15, range: 37 }, Map { dest: 37, source: 52, range: 2 }, Map { dest: 39, source: 0, range: 15 }],
            fertilizer_to_water: vec![Map { dest: 49, source: 53, range: 8 }, Map { dest: 0, source: 11, range: 42 }, Map { dest: 42, source: 0, range: 7 }, Map { dest: 57, source: 7, range: 4 }],
            water_to_light: vec![Map { dest: 88, source: 18, range: 7 }, Map { dest: 18, source: 25, range: 70 }],
            light_to_temperature: vec![Map { dest: 45, source: 77, range: 23 }, Map { dest: 81, source: 45, range: 19 }, Map { dest: 68, source: 64, range: 13 }],
            temperature_to_humidity: vec![Map { dest: 0, source: 69, range: 1 }, Map { dest: 1, source: 0, range: 69 }],
            humidity_to_location: vec![Map { dest: 60, source: 56, range: 37 }, Map { dest: 56, source: 93, range: 4 }],
        });
    }
    
    

}