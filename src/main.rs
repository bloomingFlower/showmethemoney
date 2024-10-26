mod asset_allocation;
mod economic_data;
mod models;

use asset_allocation::{adjust_allocations, calculate_risk_parity, create_asset_classes};
use chrono::NaiveDate;
use economic_data::{analyze_economic_environment, create_economic_indicators};
use std::fs;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let api_key = read_api_key_from_config()?;
    let indicators = create_economic_indicators(&api_key);
    let asset_classes = create_asset_classes();

    let start_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

    let mut current_date = start_date;
    while current_date <= end_date {
        let economic_environment = analyze_economic_environment(&indicators, current_date);
        let risk_parity_allocations = calculate_risk_parity(&asset_classes);
        let final_allocations = adjust_allocations(risk_parity_allocations, &economic_environment);

        println!("날짜: {}", current_date);
        println!("경제 환경:");
        for (indicator, value) in &economic_environment {
            println!("  {}: {:.2}", indicator, value);
        }
        println!("최종 자산 배분:");
        for decision in final_allocations {
            println!("  {}: {:.2}%", decision.asset, decision.allocation * 100.0);
        }
        println!();

        current_date = current_date.succ_opt().unwrap();
    }
    Ok(())
}

fn read_api_key_from_config() -> io::Result<String> {
    let config_path = ".config";
    let file = fs::File::open(config_path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("ALPHA_VANTAGE_API_KEY=") {
            return Ok(line.split('=').nth(1).unwrap_or("").trim().to_string());
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "API key not found in config file",
    ))
}
