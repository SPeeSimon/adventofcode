use std::{fs, str::FromStr, num::ParseIntError, ops::Range};



#[derive(Debug)]
struct Conversion {
    source_range_start: i64,
    diff: i64,
    length: i64,
}



impl FromStr for Conversion {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<i64> = s.split_whitespace().map(|d| d.parse::<i64>().unwrap()).collect();
        Ok(Conversion {
            diff: -(values[1] - values[0]),
            source_range_start: values[1],
            length: values[2],
        })
    }
}



enum MapCheck<'a> {
    NO_MATCH(&'a Range<i64>),
    MATCH(Range<i64>, Range<i64>, Range<i64>),
}



impl Conversion {
    #[inline]
    fn end(&self) -> i64 {
        self.source_range_start + self.length
    }


    fn map(&self, input: i64) -> Option<i64> {
        if input < self.source_range_start || input >= self.end() {
            return None;
        }
        return Some( input + self.diff );
    }


    fn not_in_range(&self, input: &Range<i64>) -> bool {
        input.end < self.source_range_start || input.start >= self.end()
    }


    fn map_range<'a>(&'a self, input_range: &'a Range<i64>) -> MapCheck {
        if self.not_in_range(input_range) {
            return MapCheck::NO_MATCH(input_range);
        }

        let before = Range{ start: input_range.start, end: self.source_range_start-1 };
        let within = Range{ 
                                    start: self.source_range_start.max(input_range.start) + self.diff,
                                    end: self.end().min(input_range.end) + self.diff
                                };
        let after = Range{ start: self.end(), end: input_range.end};
        return MapCheck::MATCH(before, within, after);
    }
}



#[derive(Debug)]
struct ConversionMap<'a> {
    id: &'a str,
    conversions: Vec<Conversion>,
}



impl ConversionMap<'_> {
   fn map(&self, input: i64) -> i64 {
        self.conversions.iter().filter_map(|c| c.map(input)).next().unwrap_or(input)
   }

   fn map_range(&self, input: Vec<Range<i64>>) -> Vec<Range<i64>> {
        let mut result = Vec::new();
        let mut to_remap = Vec::from_iter(input);

        // println!("map range: {}", self.id);
        // println!("    in: {:?}", &to_remap);
        // println!("   map: {:?}", &self.conversions);

        'remap: while let Some(range) = to_remap.pop() {
            for conv in &self.conversions {
                match conv.map_range(&range) {
                    MapCheck::NO_MATCH(_) => continue, // next conversion
                    MapCheck::MATCH(before, remap, after) => {
                        // print!("    ++ {:?}", &conv);
                        if !before.is_empty() {
                            // println!("    ++ remap << {:?}", &before);
                            to_remap.push(before);
                        }
                        if !after.is_empty() {
                            // println!("    ++ remap >> {:?}", &after);
                            to_remap.push(after);
                        }
                        // println!("    ++ remap == {:?}", &remap);
                        result.push(remap);
                        continue 'remap; // conversion done, next range
                    }
                }
            }
            // no conversion
            result.push(range);
        }
        return result;
   }
}



fn find_lowest_location_for_initial_seeds(input: &str) -> Vec<(i64,i64)> {
    let mut seeds = Vec::new();
    let mut mapping = Vec::new();

    for line in input.lines().into_iter() {
        if line.starts_with("seeds:") {
            seeds = line.split(":").nth(1).unwrap().split_whitespace().map(|c| c.parse::<i64>().unwrap()).collect();
            continue;
        }
        else if line.is_empty() {
            // none
        }
        else if line.ends_with("map:") {
            mapping.push(ConversionMap {
                id: line,
                conversions: Vec::new(),
            });
        }
        else if !mapping.is_empty() {
            mapping.last_mut().unwrap().conversions.push(Conversion::from_str(line).unwrap());
        }
    }

    let mut result = Vec::new();
    for seed in seeds {
        let location = mapping.iter().fold(seed, |acc, el| el.map(acc));
        result.push((seed, location));
    }
    
    return result;
}



fn find_lowest_location_for_initial_seed_range(input: &str) -> Option<i64> {
    let mut seeds = Vec::new();
    let mut mapping = Vec::new();

    for line in input.lines().into_iter() {
        if line.starts_with("seeds:") {
            let mut seed_iter = line.split(":").nth(1).unwrap().split_whitespace().map(|c| c.parse::<i64>().unwrap()).into_iter().peekable();
            while seed_iter.peek().is_some() {
                let range_start = seed_iter.next().unwrap();
                let length = seed_iter.next().unwrap();
                seeds.push(Range{start: range_start, end: range_start + length});
            }
            continue;
        }
        else if line.is_empty() {
            // none
        }
        else if line.ends_with("map:") {
            mapping.push(ConversionMap {
                id: line,
                conversions: Vec::new(),
            });
        }
        else if !mapping.is_empty() {
            mapping.last_mut().unwrap().conversions.push(Conversion::from_str(line).unwrap());
        }
    }

    println!("calculate remapping of ranges...");
    let remappings = mapping.iter()
           .fold(seeds,
                |acc, conv| conv.map_range(acc)
            );
    println!("find lowest value...");
     remappings.iter()
            .map(|remap_value| remap_value.start)
            .min()
}


#[test]
fn example1() {
    // seeds: 79, 14, 55, 13
    // 
    // seed-to-soil map:
    //   50 98 2
    //   52 50 48
    // =>
    // format: destination range start, source range start, range length
    // convert source > destination category
    // so:
    // seed 50 => soil 52   => +2
    // seed 51 => soil 53   => +2
    // ...
    // seed 97 => soil 99   => +2
    // seed 98 => soil 50   => -48
    // seed 99 => soil 51   => -48
    // unmapped?
    // source  => destination

    // find the lowest location number that corresponds to any of the initial seeds.
    // example.txt results in:
    // seed 79, soil 81, fertilizer 81, water 81, light 74, temp 78, humidity 78, location 82
    // seed 14, soil 14, fertilizer 53, water 49, light 42, temp 42, humidity 43, location 43
    // seed 55, soil 57, fertilizer 57, water 53, light 46, temp 42, humidity 82, location 86
    // seed 13, soil 13, fertilizer 52, water 41, light 34, temp 34, humidity 35, location 35
    // answer: 35

    let seed_and_locations = find_lowest_location_for_initial_seeds(include_str!("example.txt"));

    let mut verify_outcome = seed_and_locations.iter();
    assert_eq!(Some(&(79, 82)), verify_outcome.next());
    assert_eq!(Some(&(14, 43)), verify_outcome.next());
    assert_eq!(Some(&(55, 86)), verify_outcome.next());
    assert_eq!(Some(&(13, 35)), verify_outcome.next());
    println!("Lowest location number: {:?}", seed_and_locations.iter().map(|c|c.1).min());
}


#[test]
fn part1() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let seed_and_locations = find_lowest_location_for_initial_seeds(input.as_str());
    println!("Lowest location number: {:?}", seed_and_locations.iter().map(|c|c.1).min());
}


#[test]
fn example2() {
    let seed_and_locations = find_lowest_location_for_initial_seed_range(include_str!("example.txt"));
    println!("Lowest location number: \x1b[32m{}\x1b[0m", seed_and_locations.unwrap());
    assert_eq!(Some(46), seed_and_locations);
}


#[test]
fn part2() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    let seed_and_locations = find_lowest_location_for_initial_seed_range(input.as_str());
    println!("Lowest location number: \x1b[32m{}\x1b[0m", seed_and_locations.unwrap());
}
