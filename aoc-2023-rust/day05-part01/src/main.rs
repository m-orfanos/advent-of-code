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
                // seeds: 79 14 55 13
                let parts = parse_heading(&line);
                seeds = parse_arr(parts[1]);
                mode = ParseMode::Pending;
            }
            // config format
            // seed-to-soil map:
            // 50 98 2
            // 52 50 48
            ParseMode::SeedToSoil => extend_cfg(&mut seed_to_soil, parse_arr(&line)),
            ParseMode::SoilToFertilizer => extend_cfg(&mut soil_to_fert, parse_arr(&line)),
            ParseMode::FertilizerToWater => extend_cfg(&mut fert_to_water, parse_arr(&line)),
            ParseMode::WaterToLight => extend_cfg(&mut water_to_light, parse_arr(&line)),
            ParseMode::LightToTemperature => extend_cfg(&mut light_to_temp, parse_arr(&line)),
            ParseMode::TemperatureToHumidity => extend_cfg(&mut temp_to_hum, parse_arr(&line)),
            ParseMode::HumitityToLocation => extend_cfg(&mut hum_to_loc, parse_arr(&line)),
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

fn extend_cfg(map: &mut Vec<Config>, config: Vec<u64>) {
    map.push(Config {
        src: config[1],
        dst: config[0],
        distance: config[2],
    });
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
