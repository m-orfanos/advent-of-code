use std::io::{self, BufRead};

enum ParseMode {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumitityToLocation,
    Pending,
}

fn main() {
    let mut seeds: Vec<(u64, u64)> = Vec::new();

    let mut seed_to_soil: Vec<(u64, u64, u64)> = vec![];
    let mut soil_to_fert: Vec<(u64, u64, u64)> = vec![];
    let mut fert_to_water: Vec<(u64, u64, u64)> = vec![];
    let mut water_to_light: Vec<(u64, u64, u64)> = vec![];
    let mut light_to_temp: Vec<(u64, u64, u64)> = vec![];
    let mut temp_to_hum: Vec<(u64, u64, u64)> = vec![];
    let mut hum_to_loc: Vec<(u64, u64, u64)> = vec![];

    let mut mode = ParseMode::Seeds;

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        if line.is_empty() {
            // reached the end of the current config
            // reset mode
            mode = ParseMode::Pending;
            continue;
        }

        match mode {
            ParseMode::Seeds => {
                // seeds: 79 14 55 13
                let parts = parse_heading(&line);
                let config = parse_arr(parts[1]);

                let mut it = config.iter();
                while let Some(src_start) = it.next() {
                    seeds.push((src_start + 0, it.next().unwrap() + 0));
                }

                mode = ParseMode::Pending;
            }
            // config format
            // seed-to-soil map:
            // 50 98 2
            // 52 50 48
            ParseMode::SeedToSoil => {
                let config = parse_arr(&line);
                seed_to_soil.push((config[1], config[0], config[2]));
            }
            ParseMode::SoilToFertilizer => {
                let config = parse_arr(&line);
                soil_to_fert.push((config[1], config[0], config[2]));
            }
            ParseMode::FertilizerToWater => {
                let config = parse_arr(&line);
                fert_to_water.push((config[1], config[0], config[2]));
            }
            ParseMode::WaterToLight => {
                let config = parse_arr(&line);
                water_to_light.push((config[1], config[0], config[2]));
            }
            ParseMode::LightToTemperature => {
                let config = parse_arr(&line);
                light_to_temp.push((config[1], config[0], config[2]));
            }
            ParseMode::TemperatureToHumidity => {
                let config = parse_arr(&line);
                temp_to_hum.push((config[1], config[0], config[2]));
            }
            ParseMode::HumitityToLocation => {
                let config = parse_arr(&line);
                hum_to_loc.push((config[1], config[0], config[2]));
            }
            // determine what should be parsed next
            // empty lines are ignored
            ParseMode::Pending => {
                if line.is_empty() {
                    mode = ParseMode::Pending;
                } else if line.starts_with("seeds") {
                    mode = ParseMode::Seeds;
                } else if line.starts_with("seed-to-soil") {
                    mode = ParseMode::SeedToSoil;
                } else if line.starts_with("soil-to-fertilizer") {
                    mode = ParseMode::SoilToFertilizer;
                } else if line.starts_with("fertilizer-to-water") {
                    mode = ParseMode::FertilizerToWater;
                } else if line.starts_with("water-to-light") {
                    mode = ParseMode::WaterToLight;
                } else if line.starts_with("light-to-temperature") {
                    mode = ParseMode::LightToTemperature;
                } else if line.starts_with("temperature-to-humidity") {
                    mode = ParseMode::TemperatureToHumidity;
                } else if line.starts_with("humidity-to-location") {
                    mode = ParseMode::HumitityToLocation;
                } else {
                    mode = ParseMode::Pending;
                }
            }
        }
    }

    fill_missing_ranges(&mut seed_to_soil);
    fill_missing_ranges(&mut soil_to_fert);
    fill_missing_ranges(&mut fert_to_water);
    fill_missing_ranges(&mut water_to_light);
    fill_missing_ranges(&mut light_to_temp);
    fill_missing_ranges(&mut temp_to_hum);
    fill_missing_ranges(&mut hum_to_loc);

    let soils = find_next(&seeds, seed_to_soil);
    let ferts = find_next(&soils, soil_to_fert);
    let waters = find_next(&ferts, fert_to_water);
    let lights = find_next(&waters, water_to_light);
    let temps = find_next(&lights, light_to_temp);
    let hums = find_next(&temps, temp_to_hum);
    let locs = find_next(&hums, hum_to_loc);

    // calculate result
    let mut ans = u64::MAX;
    for (l, _) in locs {
        if l < ans {
            ans = l;
        }
    }

    println!("{:?}", ans);
}

fn find_next(srcs: &Vec<(u64, u64)>, src_to_dst: Vec<(u64, u64, u64)>) -> Vec<(u64, u64)> {
    // traverse the various mappings and
    // return the next inputs, which are fed
    // forward to the next mappings, etc.
    let mut dsts = Vec::new();
    for src in srcs {
        // reminder: there are no holes in the mappings' ranges
        // ranges must be continuous
        for s2s in &src_to_dst {
            // attempt to find where the src intersects the mapping
            let x = find_intersection(s2s.0, s2s.0 + s2s.2, src.0, src.0 + src.1);
            match x {
                // an intersection was found
                // add a "destination" range
                // this will be used as the next "source" range
                Some((s, e)) => {
                    dsts.push((s2s.1 + (s - s2s.0), e - s));
                }
                None => (),
            }
        }
    }
    dsts
}

fn fill_missing_ranges(v: &mut Vec<(u64, u64, u64)>) {
    let mut missing = find_missing_ranges(&*v);
    v.append(&mut missing);
}

fn find_missing_ranges(v: &Vec<(u64, u64, u64)>) -> Vec<(u64, u64, u64)> {
    // there is a unique mapping for every A->B
    // but we only care about a mapping's range
    // the parsed ranges have many "holes" in them
    // we fill in these holds manually
    let mut parsed_ranges = vec![];
    for ss in v {
        parsed_ranges.push((ss.0, ss.1, ss.2));
    }
    parsed_ranges.sort_by_key(|x| x.0);

    // no parsed ranges, return "[-inf, +inf]"
    if parsed_ranges.is_empty() {
        return vec![(u64::MIN, u64::MIN, u64::MAX)];
    }

    // determine the missing ranges
    // all ranges must be within [u64::MIN, u64::MAX]
    let mut missing_ranges: Vec<(u64, u64, u64)> = Vec::new();
    let mut cur = u64::MIN;
    for (src, _, dist) in parsed_ranges.iter() {
        if src + 0 == 0 || src - cur == 0 {
            cur = src + dist;
            continue;
        }
        missing_ranges.push((cur, cur, src - cur));
        cur = src + dist;
    }

    // add right-end to "infinity"
    match parsed_ranges.last() {
        Some((s, _, r)) => {
            missing_ranges.push((s + r, s + r, u64::MAX - (s + r)));
        }
        None => (),
    }
    missing_ranges
}

fn find_intersection(s1: u64, e1: u64, s2: u64, e2: u64) -> Option<(u64, u64)> {
    if s2 > e1 || s1 > e2 {
        None
    } else {
        let s = s1.max(s2);
        let e = e1.min(e2);
        Some((s, e))
    }
}

// formats
// seeds: 79 14 55 13
// seed-to-soil map:
fn parse_heading(s: &String) -> Vec<&str> {
    s.split(":").map(|x| x.trim()).collect::<Vec<&str>>()
}

// format
// 79 14 55 13
fn parse_arr(s: &str) -> Vec<u64> {
    s.split(" ")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
