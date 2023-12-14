use crates::common::day05::ParseMode;
use crates::parsers::{parse_u64s, split};
use std::io::{self, BufRead};

struct Config {
    src: u64,
    dst: u64,
    distance: u64,
}

fn main() {
    let mut seeds: Vec<u64> = Vec::new();

    let mut seed_to_soil: Vec<Config> = vec![];
    let mut soil_to_fert: Vec<Config> = vec![];
    let mut fert_to_water: Vec<Config> = vec![];
    let mut water_to_light: Vec<Config> = vec![];
    let mut light_to_temp: Vec<Config> = vec![];
    let mut temp_to_hum: Vec<Config> = vec![];
    let mut hum_to_loc: Vec<Config> = vec![];

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
                seeds = parse_seeds(&line);
                mode = ParseMode::Pending;
            }
            ParseMode::SeedToSoil => insert(&mut seed_to_soil, &line),
            ParseMode::SoilToFertilizer => insert(&mut soil_to_fert, &line),
            ParseMode::FertilizerToWater => insert(&mut fert_to_water, &line),
            ParseMode::WaterToLight => insert(&mut water_to_light, &line),
            ParseMode::LightToTemperature => insert(&mut light_to_temp, &line),
            ParseMode::TemperatureToHumidity => insert(&mut temp_to_hum, &line),
            ParseMode::HumidityToLocation => insert(&mut hum_to_loc, &line),
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
                    mode = ParseMode::HumidityToLocation;
                } else {
                    mode = ParseMode::Pending;
                }
            }
        }
    }

    let mut ans = u64::MAX;
    for seed in seeds {
        let soil = find_next(seed, &seed_to_soil);
        let fert = find_next(soil, &soil_to_fert);
        let water = find_next(fert, &fert_to_water);
        let light = find_next(water, &water_to_light);
        let temp = find_next(light, &light_to_temp);
        let hum = find_next(temp, &temp_to_hum);
        let loc = find_next(hum, &hum_to_loc);

        if loc < ans {
            ans = loc;
        }
    }

    println!("{}", ans);
}

// TODO check lifetimes in this file
fn insert(src_to_dst: &mut Vec<Config>, line: &String) {
    let config = parse_u64s(&line, " ");
    src_to_dst.push(Config {
        src: config[1],
        dst: config[0],
        distance: config[2],
    });
}

fn parse_seeds(line: &String) -> Vec<u64> {
    parse_u64s(&split(&line, ":")[1], " ")
}

fn find_next(seed: u64, cx: &Vec<Config>) -> u64 {
    let mut needle = seed;
    for c in cx {
        if needle < c.src {
            continue;
        }
        if c.src <= needle && needle <= c.src + c.distance {
            needle = c.dst + (needle - c.src);
            break;
        }
    }
    needle
}
