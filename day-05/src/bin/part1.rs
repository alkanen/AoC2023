use indicatif::ProgressBar;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part1(input);
    dbg!(output);
}

enum Maps {
    Seed2Soil = 0,
    Soil2Fertilizer,
    Fertilizer2Water,
    Water2Light,
    Light2Temperature,
    Temperature2Humidity,
    Humidity2Location
}

fn create_seed_list(input: &str) -> Vec<usize> {
    let mut seed_list = Vec::new();
    for line in input.lines() {
        let (_label, seeds_str) = line.split_once(": ").unwrap();

        let seeds = seeds_str.split_whitespace().collect::<Vec<&str>>();
        for seed in seeds.iter() {
            seed_list.push(seed.parse::<usize>().unwrap());
        }

        break;
    }

    return seed_list;
}

fn create_map_lists(lines: &Vec<&str>) -> (usize, Vec<Vec<(usize, usize, usize)>>) {
    let mut maps: Vec<Vec<(usize, usize, usize)>> = Vec::new();
    for _i in 0..7 {
        maps.push(Vec::new());
    }

    let mut highest_number: usize = 0;

    const STATE_START: usize = 0;
    const STATE_MAPPING: usize = 1;

    let mut state = STATE_START;
    let mut currentmap: usize = Maps::Seed2Soil as usize;
    for (i, line) in lines.iter().enumerate() {
        // Skip seeds and empty line
        if i < 2 {
            continue;
        }

        // println!("Line: {}", line);
        if state == STATE_START {
            state = STATE_MAPPING;
            if *line == "seed-to-soil map:" {
                currentmap = Maps::Seed2Soil as usize;
            }
            else if *line == "soil-to-fertilizer map:" {
                currentmap = Maps::Soil2Fertilizer as usize;
            }
            else if *line == "fertilizer-to-water map:" {
                currentmap = Maps::Fertilizer2Water as usize;
            }
            else if *line == "water-to-light map:" {
                currentmap = Maps::Water2Light as usize;
            }
            else if *line == "light-to-temperature map:" {
                currentmap = Maps::Light2Temperature as usize;
            }
            else if *line == "temperature-to-humidity map:" {
                currentmap = Maps::Temperature2Humidity as usize;
            }
            else if *line == "humidity-to-location map:" {
                currentmap = Maps::Humidity2Location as usize;
            }
            else {
                panic!("Unknown state transition at line {}: {}", i, line);
            }
        } else if state == STATE_MAPPING {
            if *line == "" {
                state = STATE_START;
            } else {
                let parts = line.split(" ").collect::<Vec<&str>>();                
                let destination = parts[0].parse::<usize>().unwrap();
                let source = parts[1].parse::<usize>().unwrap();
                let range = parts[2].parse::<usize>().unwrap();
                maps[currentmap].push((destination, source, range));

                if destination + range - 1 > highest_number {
                    highest_number = destination + range - 1;
                }
                if source + range - 1 > highest_number {
                    highest_number = source + range - 1;
                }
            }
        }
    }
       
    return (highest_number, maps);   
}

fn map_destination_to_source(destination: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    let mut source = 0;
    let mut found = false;

    for (d, s, r) in map.iter() {
        let d_end = d + r;
        // let s_end = s + r;

        if destination >= *d && destination < d_end {
            source = *s + (destination - *d);
            found = true;
            break;
        }
    }

    if found {
        return source;
    } else {
        // If target doesn't exist in the map, it's the same as the source.
        return destination;
    }
}

fn map_source_to_destination(source: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    let mut destination = 0;
    let mut found = false;

    for (d, s, r) in map.iter() {
        //let d_end = d + r;
        let s_end = s + r;

        if source >= *s && source < s_end {
            destination = *d + (source - *s);
            found = true;
            break;
        }
    }

    if found {
        return destination;
    } else {
        // If target doesn't exist in the map, it's the same as the source.
        return source;
    }
}
fn part1(input: &str) -> String {
    let seed_list = create_seed_list(input);
    let lines = input.lines().collect::<Vec<&str>>();
    let (_highest_number, maps) = create_map_lists(&lines);

    let mut best_location = 0xffffffffffffffff;

    let bar = ProgressBar::new(seed_list.len() as u64);
    for seed in seed_list.iter() {
        bar.inc(1);
        println!("Seed: {}", seed);

        // Go from seed all the way to location
        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.

        let soil = map_source_to_destination(*seed, &maps[Maps::Seed2Soil as usize]);
        println!("  Seed -> soil - {}: {}", seed, soil);

        let fertilizer = map_source_to_destination(soil, &maps[Maps::Soil2Fertilizer as usize]);
        println!("  Soil -> fertilizer - {}: {}", soil, fertilizer);

        let water = map_source_to_destination(fertilizer, &maps[Maps::Fertilizer2Water as usize]);
        println!("  Fertilizer -> water - {}: {}", fertilizer, water);

        let light = map_source_to_destination(water, &maps[Maps::Water2Light as usize]);
        println!("  Water -> light - {}: {}", water, light);

        let temperature = map_source_to_destination(light, &maps[Maps::Light2Temperature as usize]);
        println!("  Light -> temperature - {}: {}", light, temperature);

        let humidity = map_source_to_destination(temperature, &maps[Maps::Temperature2Humidity as usize]);
        println!("  Temperature -> humidity - {}: {}", temperature, humidity);

        let location = map_source_to_destination(humidity, &maps[Maps::Humidity2Location as usize]);
        println!("  Humidity -> location - {}: {}", humidity, location);

        if location < best_location {
            best_location = location;
        }

    }
    bar.finish();

    return best_location.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

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
";

    #[test]
    fn create_seed_list_test() {
        let result = create_seed_list(INPUT);
        assert_eq!(result, vec![79, 14, 55, 13]);
    }

    #[test]
    fn map_destination_to_source_test() {
        let map = vec![(50, 98, 2), (52, 50, 48)];

        //for (d, s) in vec![(50, 98), (51, 99), (100, 100), (55, 53), (10, 10)].iter() {
        for (d, s) in vec![
            (0, 0),
            (1, 1),
            // ...
            (48, 48),
            (49, 49),
            (52, 50),
            (53, 51),
            // ...
            (98, 96),
            (99, 97),
            (50, 98),
            (51, 99)
        ].iter() {
            let source = map_destination_to_source(*d, &map);
            assert_eq!(source, *s);
        }
    }

    #[test]
    fn create_map_lists_test() {
        let lines = INPUT.lines().collect::<Vec<&str>>();

        let (high, result) = create_map_lists(&lines);
        assert_eq!(high, 99);
        // Result: [[(50, 98, 2), (52, 50, 48)], [(0, 15, 37), (37, 52, 2), (39, 0, 15)], [(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)], [(88, 18, 7), (18, 25, 70)], [(45, 77, 23), (81, 45, 19), (68, 64, 13)], [(0, 69, 1), (1, 0, 69)], [(60, 56, 37), (56, 93, 4)]]
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(result[1], vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]);
        assert_eq!(result[2], vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]);
        assert_eq!(result[3], vec![(88, 18, 7), (18, 25, 70)]);
        assert_eq!(result[4], vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]);
        assert_eq!(result[5], vec![(0, 69, 1), (1, 0, 69)]);
        assert_eq!(result[6], vec![(60, 56, 37), (56, 93, 4)]);
    }

    #[test]
    fn it_works1() {
        let result = part1(INPUT);
        assert_eq!(result, "35".to_string());
    }
}