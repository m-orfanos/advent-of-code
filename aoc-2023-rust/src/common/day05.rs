pub enum ParseMode {
    Seeds,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Pending,
}

pub fn get_mode(line: String) -> ParseMode {
    if line.is_empty() {
        ParseMode::Pending
    } else if line.starts_with("seeds") {
        ParseMode::Seeds
    } else if line.starts_with("seed-to-soil") {
        ParseMode::SeedToSoil
    } else if line.starts_with("soil-to-fertilizer") {
        ParseMode::SoilToFertilizer
    } else if line.starts_with("fertilizer-to-water") {
        ParseMode::FertilizerToWater
    } else if line.starts_with("water-to-light") {
        ParseMode::WaterToLight
    } else if line.starts_with("light-to-temperature") {
        ParseMode::LightToTemperature
    } else if line.starts_with("temperature-to-humidity") {
        ParseMode::TemperatureToHumidity
    } else if line.starts_with("humidity-to-location") {
        ParseMode::HumidityToLocation
    } else {
        ParseMode::Pending
    }
}
