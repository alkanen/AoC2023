use indicatif::ProgressBar;
use indicatif::MultiProgress;
use std::time::Duration;

fn main() {
    let input = include_str!("../../input.txt");
    let output = part2(input);
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

#[derive(PartialEq, Eq, Debug)]
struct Map {
    destination: usize,
    source: usize,
    range: usize
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
struct ResourceRange {
    start: usize,
    end: usize,
}

fn create_seed_list(input: &str) -> Vec<ResourceRange> {
    let mut seed_list = Vec::new();
    for line in input.lines() {
        let (_label, seeds_str) = line.split_once(": ").unwrap();

        let seeds = seeds_str.split_whitespace().collect::<Vec<&str>>();
        for i in 0..seeds.len() {
            if i % 2 == 1 {
                continue;
            }
            let seed = seeds[i].parse::<usize>().unwrap();
            let range = seeds[i + 1].parse::<usize>().unwrap();
            seed_list.push(ResourceRange{ start: seed, end: seed + range });
        }

        break;
    }

    return seed_list;
}

fn create_map_lists(lines: &Vec<&str>) -> (usize, Vec<Vec<Map>>) {
    let mut maps: Vec<Vec<Map>> = Vec::new();
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
                maps[currentmap].push(Map{destination: destination, source: source, range:range});

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

fn map_sources_to_destinations(sources: &Vec<ResourceRange>, map: &Vec<Map>, mp: Option<&MultiProgress>) -> Vec<ResourceRange> {
    // Takes a list of start and end points, and a map, and returns a list of start and
    // end points, taking care to handle splitting of ranges when appropriate.
    // The last element of both `sources` and returned vector elements is the location
    // associated with the tuple's start point.

    let debug = false;

    let mut destinations: Vec<ResourceRange> = Vec::new();
    let mut local_sources = sources.clone();

    let mut ctr: usize = 0;
    let bar: ProgressBar;
    
    if let Some(mp) = mp {
        bar = mp.add(ProgressBar::new(local_sources.len() as u64));
    } else {
        bar = ProgressBar::new(local_sources.len() as u64);
    }

    loop {
        if ctr >= local_sources.len() {
            bar.finish();
            break;
        }
        
        let source = local_sources[ctr];
        if debug {println!("Source {}: {} -> {}", ctr, source.start, source.end);}
        ctr += 1;

        let mut added: bool = false;

        for map in map.iter() {
            if debug {println!(
                "  Map: d {} -> {} ({}), s {} -> {} ({})", 
                map.destination, map.destination + map.range, map.range,
                map.source, map.source + map.range, map.range
            );}
            
            // If the source range is completely before the map range, skip it.
            if source.end < map.source {
                if debug {println!("  Skipping {} -> {} because it's before the map range", source.start, source.end);}
                continue;                
            }

            // If the source range is completely after the map range, skip it.
            if source.start >= map.source + map.range {
                if debug {println!("  Skipping {} -> {} because it's after the map range", source.start, source.end);}
                continue;                
            }

            // If the source range is completely within the map range, add the
            // destination range to the list.
            if source.start >= map.source && source.end <= map.source + map.range {
                if debug {println!("  Adding {} -> {} because it's within the map range", map.destination, map.destination + map.range);}
                destinations.push(
                    ResourceRange{
                        start: map.destination + (source.start - map.source),
                        end: map.destination + (source.end - map.source)
                    }
                );
                added = true;
                break;
            }

            // If the source range is partially before and partially after the
            // map range, add the destination range to the list, and add the
            // remaining source ranges to the list.
            if source.start < map.source && source.end > map.source + map.range {
                if debug {println!(
                    "  Adding {} -> {} because it's partially before and partially after the map range", 
                    map.destination, map.destination + map.range
                );}
                destinations.push(
                    ResourceRange{
                        start: map.destination,
                        end: map.destination + map.range
                    }
                );
                local_sources.push(
                    ResourceRange{
                        start: source.start,
                        end: map.source
                    }
                );
                local_sources.push(
                    ResourceRange{
                        start: map.source + map.range,
                        end: source.end
                    }
                );
                added = true;
                break;
            }

            // If the source range is partially before and partially within the
            // map range, add the destination range to the list, and add the
            // remaining source range to the list.
            if source.start < map.source && source.end > map.source && source.end <= map.source + map.range {
                if debug {println!(
                    "  Adding {} -> {} because it's partially before and partially within the map range",
                    map.destination, map.destination + map.range
                );

                println!(
                    "    soure.start {} < map.source {} && source.end {} <= map.source + map.range {}",
                    source.start, map.source, source.end, map.source + map.range
                );}
                destinations.push(
                    ResourceRange{
                        start: map.destination,
                        end: map.destination + (source.end - map.source)
                    }
                );
                local_sources.push(
                    ResourceRange{
                        start: source.start,
                        end: map.source
                    }
                );
                added = true;
                break;
            }

            // If the source range is partially within and partially after the
            // map range, add the destination range to the list, and add the
            // remaining source range to the list.
            if source.start >= map.source && source.start < map.source + map.range && source.end > map.source + map.range {
                if debug {println!(
                    "  Adding {} -> {} because it's partially within and partially after the map range",
                    map.destination + (source.start - map.source), map.destination + map.range
                );}
                destinations.push(
                    ResourceRange{
                        start: map.destination + (source.start - map.source),
                        end: map.destination + map.range
                    }
                );
                local_sources.push(
                    ResourceRange{
                        start: map.source + map.range,
                        end: source.end
                    }
                );
                added = true;
                break;
            }

        }

        if !added {
            if debug {println!("  LAST Adding {} -> {} because there was no match", source.start, source.end);}
            destinations.push(ResourceRange{start: source.start, end: source.end});
        }

        bar.inc(1);
    }

    if debug {
        for dest in destinations.iter() {
            println!("  Dest: {} -> {}", dest.start, dest.end);
        }
    }

    return destinations;
}

fn part2(input: &str) -> String {
    let seed_list = create_seed_list(input);
    let mut total_range: usize = 0;
    for seed_range in seed_list.iter() {
        println!("Seed: {} -> {}", seed_range.start, seed_range.end);
        total_range += seed_range.end - seed_range.start;
    }

    println!("Total range: {}", total_range);

    let lines = input.lines().collect::<Vec<&str>>();
    let (_high, maps) = create_map_lists(&lines);

    let mp = MultiProgress::new();
    let bar = mp.add(ProgressBar::new(seed_list.len() as u64));
    
    bar.enable_steady_tick(Duration::from_millis(100));

    let mut best_location: i64 = -1;

    let mut new_range: usize = 0;
    for seed_range in seed_list.iter() {
        println!("Seed: {} -> {}", seed_range.start, seed_range.end);

        // Go from seed all the way to location
        // Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.

        let soils = map_sources_to_destinations(&vec![*seed_range], &maps[Maps::Seed2Soil as usize], Some(&mp));
        for soil_range in soils.iter() {
            println!("  Soil: {} -> {}", soil_range.start, soil_range.end);
        }

        let fertilizers = map_sources_to_destinations(&soils, &maps[Maps::Soil2Fertilizer as usize], Some(&mp));
        for fertilizer_range in fertilizers.iter() {
            println!("  Fertilizer: {} -> {}", fertilizer_range.start, fertilizer_range.end);
        }

        let waters = map_sources_to_destinations(&fertilizers, &maps[Maps::Fertilizer2Water as usize], Some(&mp));
        for water_range in waters.iter() {
            println!("  Water: {} -> {}", water_range.start, water_range.end);
        }

        let lights = map_sources_to_destinations(&waters, &maps[Maps::Water2Light as usize], Some(&mp));
        for light_range in lights.iter() {
            println!("  Light: {} -> {}", light_range.start, light_range.end);
        }

        let temperatures = map_sources_to_destinations(&lights, &maps[Maps::Light2Temperature as usize], Some(&mp));
        for temperature_range in temperatures.iter() {
            println!("  Temperature: {} -> {}", temperature_range.start, temperature_range.end);
        }

        let humidities = map_sources_to_destinations(&temperatures, &maps[Maps::Temperature2Humidity as usize], Some(&mp));
        for humidity_range in humidities.iter() {
            println!("  Humidity: {} -> {}", humidity_range.start, humidity_range.end);
        }

        let locations = map_sources_to_destinations(&humidities, &maps[Maps::Humidity2Location as usize], Some(&mp));
        for location_range in locations.iter() {
            println!("  Location: {} -> {}", location_range.start, location_range.end);

            if best_location == -1 || location_range.start < best_location as usize {
                best_location = location_range.start as i64;
            }
        }

        bar.inc(1);
        //break;
    }

    //println!("New range: {}", new_range);
    //assert_eq!(new_range, total_range);

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
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 79, end: 93},
                ResourceRange{start: 55, end: 68}
            ]
        );
    }

    #[test]
    fn create_map_lists_test() {
        let lines = INPUT.lines().collect::<Vec<&str>>();

        let (high, result) = create_map_lists(&lines);
        assert_eq!(high, 99);
        // Result: [[(50, 98, 2), (52, 50, 48)], [(0, 15, 37), (37, 52, 2), (39, 0, 15)], [(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)], [(88, 18, 7), (18, 25, 70)], [(45, 77, 23), (81, 45, 19), (68, 64, 13)], [(0, 69, 1), (1, 0, 69)], [(60, 56, 37), (56, 93, 4)]]
        assert_eq!(result.len(), 7);
        assert_eq!(result[0], vec![Map{destination: 50 ,source: 98, range:  2}, Map{destination: 52, source: 50, range: 48}]);
        assert_eq!(
            result[1],
            vec![
                Map{destination:  0, source: 15, range: 37},
                Map{destination: 37, source: 52, range:  2},
                Map{destination: 39, source:  0, range: 15}
            ]
        );
        assert_eq!(
            result[2],
            vec![
                Map{destination: 49, source: 53, range:  8},
                Map{destination:  0, source: 11, range: 42},
                Map{destination: 42, source:  0, range:  7},
                Map{destination: 57, source:  7, range:  4}
            ]
        );
        assert_eq!(result[3], vec![Map{destination: 88, source: 18, range:  7}, Map{destination: 18, source: 25, range: 70}]);
        assert_eq!(
            result[4],
            vec![
                Map{destination: 45, source: 77, range: 23},
                Map{destination: 81, source: 45, range: 19},
                Map{destination: 68, source: 64, range: 13}
            ]
        );
        assert_eq!(result[5], vec![Map{destination:  0, source: 69, range:  1}, Map{destination:  1, source:  0, range: 69}]);
        assert_eq!(result[6], vec![Map{destination: 60, source: 56, range: 37}, Map{destination: 56, source: 93, range:  4}]);
    }

    #[test]
    fn before() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 90, end: 95}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 90, end: 95}
            ]
        );
    }

    #[test]
    fn before_tangent() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 90, end: 100}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 90, end: 100}
            ]
        );
    }

    #[test]
    fn after() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 120, end: 125}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 120, end: 125}
            ]
        );
    }

    #[test]
    fn after_tangent() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 110, end: 120}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 110, end: 120}
            ]
        );
    }

    #[test]
    fn within() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 101, end: 109}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 201, end: 209}
            ]
        );
    }

    #[test]
    fn within_tangent_start() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 100, end: 109}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 200, end: 209}
            ]
        );
    }

    #[test]
    fn within_tangent_end() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 101, end: 110}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 201, end: 210  }
            ]
        );
    }

    #[test]
    fn outside() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 90, end: 120}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 200, end: 210},
                ResourceRange{start: 90, end: 100},
                ResourceRange{start: 110, end: 120}
            ]
        );
    }

    #[test]
    fn outside_before() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 90, end: 105}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 200, end: 205},
                ResourceRange{start: 90, end: 100}
            ]
        );
    }

    #[test]
    fn outside_before_tangent() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 90, end: 110}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 200, end: 210},
                ResourceRange{start: 90, end: 100}
            ]
        );
    }

    #[test]
    fn outside_after() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 105, end: 115}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 205, end: 210},
                ResourceRange{start: 110, end: 115}
            ]
        );
    }

    #[test]
    fn outside_after_tangent() {
        let maps = vec![
            Map{destination: 200, source: 100, range: 10}
        ];

        let sources = vec![ResourceRange{start: 100, end: 115}];
        let result = map_sources_to_destinations(&sources, &maps, None); 
        assert_eq!(
            result,
            vec![
                ResourceRange{start: 200, end: 210},
                ResourceRange{start: 110, end: 115}
            ]
        );
    }

    /**/
    #[test]
    fn it_works2() {
        let result = part2(INPUT);
        assert_eq!(result, "46".to_string());
    }
    /**/
}