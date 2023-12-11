use std::{
    collections::HashMap,
    io::{self, BufRead},
};

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
    let mut seeds: Vec<u64> = Vec::new();
    let mut seed_to_soil: HashMap<u64, u64> = HashMap::new();
    let mut soil_to_fert: HashMap<u64, u64> = HashMap::new();
    let mut fert_to_water: HashMap<u64, u64> = HashMap::new();
    let mut water_to_light: HashMap<u64, u64> = HashMap::new();
    let mut light_to_temp: HashMap<u64, u64> = HashMap::new();
    let mut temp_to_hum: HashMap<u64, u64> = HashMap::new();
    let mut hum_to_loc: HashMap<u64, u64> = HashMap::new();

    let mut mode = ParseMode::Seeds;

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        if line.is_empty() {
            mode = ParseMode::Pending;
            println!("DONE PARSING");
            continue;
        }

        match mode {
            ParseMode::Seeds => {
                // seeds: 79 14 55 13
                let parts = parse_heading(&line);
                seeds = parse_arr(parts[1]);
                mode = ParseMode::Pending;
            }
            ParseMode::SeedToSoil => extend_map(&mut seed_to_soil, parse_arr(&line)),
            ParseMode::SoilToFertilizer => extend_map(&mut soil_to_fert, parse_arr(&line)),
            ParseMode::FertilizerToWater => extend_map(&mut fert_to_water, parse_arr(&line)),
            ParseMode::WaterToLight => extend_map(&mut water_to_light, parse_arr(&line)),
            ParseMode::LightToTemperature => extend_map(&mut light_to_temp, parse_arr(&line)),
            ParseMode::TemperatureToHumidity => extend_map(&mut temp_to_hum, parse_arr(&line)),
            ParseMode::HumitityToLocation => extend_map(&mut hum_to_loc, parse_arr(&line)),
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

    println!("DONE PARSING");

    let mut ans = u64::MAX;
    for seed in seeds {
        let soil = seed_to_soil.get(&seed).or(Some(&seed)).unwrap();
        let fert = soil_to_fert.get(&soil).or(Some(&soil)).unwrap();
        let water = fert_to_water.get(&fert).or(Some(&fert)).unwrap();
        let light = water_to_light.get(&water).or(Some(&water)).unwrap();
        let temp = light_to_temp.get(&light).or(Some(&light)).unwrap();
        let humidity = temp_to_hum.get(&temp).or(Some(&temp)).unwrap();
        let location = hum_to_loc.get(&humidity).or(Some(&humidity)).unwrap();

        if *location < ans {
            ans = *location;
        }
    }

    println!("{}", ans);
}

fn extend_map(map: &mut HashMap<u64, u64>, config: Vec<u64>) {
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    let dst_start = config[0];
    let src_start = config[1];
    let range_length = config[2];

    // seed  soil
    // 0     0
    // 1     1
    // ...   ...
    // 48    48
    // 49    49
    // 50    52
    // 51    53
    // ...   ...
    // 96    98
    // 97    99
    // 98    50
    // 99    51
    for i in 0..range_length {
        let src = src_start + i;
        let dst = dst_start + i;
        map.insert(src, dst);
    }
}

fn parse_heading(s: &String) -> Vec<&str> {
    s.split(":").map(|x| x.trim()).collect::<Vec<&str>>()
}

fn parse_arr(s: &str) -> Vec<u64> {
    s.split(" ")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
