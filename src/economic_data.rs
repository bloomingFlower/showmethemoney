use crate::models::{AlphaVantageResponse, EconomicIndicator};
use chrono::NaiveDate;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;

pub fn fetch_economic_data(
    indicator: &str,
    api_key: &str,
) -> Result<Vec<(NaiveDate, f64)>, Box<dyn Error>> {
    let client = Client::new();
    let url = format!(
        "https://www.alphavantage.co/query?function={}&interval=monthly&apikey={}",
        indicator, api_key
    );

    let response: AlphaVantageResponse = client.get(&url).send()?.json()?;

    let mut data = Vec::new();
    for point in response.data {
        if let (Ok(date), Ok(value)) = (
            NaiveDate::parse_from_str(&point.date, "%Y-%m-%d"),
            point.value.parse::<f64>(),
        ) {
            data.push((date, value));
        }
    }

    Ok(data)
}

pub fn create_economic_indicators(api_key: &str) -> Vec<EconomicIndicator> {
    let indicators = vec![
        ("REAL_GDP", "GDP Growth", 0.2),
        ("INFLATION", "Inflation Rate", 0.2),
        ("UNEMPLOYMENT", "Unemployment Rate", 0.15),
        ("FEDERAL_FUNDS_RATE", "Interest Rate", 0.15),
        ("CONSUMER_SENTIMENT", "Consumer Confidence", 0.1),
        ("NONFARM_PAYROLL", "Employment", 0.1),
        ("RETAIL_SALES", "Retail Sales", 0.1),
    ];

    indicators
        .into_iter()
        .filter_map(|(av_indicator, name, weight)| {
            match fetch_economic_data(av_indicator, api_key) {
                Ok(data) => Some(EconomicIndicator {
                    name: name.to_string(),
                    data,
                    weight,
                }),
                Err(e) => {
                    eprintln!("Failed to fetch data for {}: {}", name, e);
                    None
                }
            }
        })
        .collect()
}

pub fn analyze_economic_environment(
    indicators: &[EconomicIndicator],
    date: NaiveDate,
) -> HashMap<String, f64> {
    let mut environment = HashMap::new();

    for indicator in indicators {
        if let Some((_, value)) = indicator.data.iter().find(|&&(d, _)| d == date) {
            environment.insert(indicator.name.clone(), *value);
        }
    }

    environment
}
