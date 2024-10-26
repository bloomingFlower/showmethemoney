use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;

pub struct EconomicIndicator {
    pub name: String,
    pub data: Vec<(NaiveDate, f64)>,
    pub weight: f64,
}

pub struct AssetClass {
    pub name: String,
    pub expected_return: f64,
    pub volatility: f64,
    pub correlation: HashMap<String, f64>,
}

pub struct InvestmentDecision {
    pub asset: String,
    pub allocation: f64,
}

#[derive(Deserialize)]
pub struct AlphaVantageResponse {
    pub data: Vec<EconomicDataPoint>,
}

#[derive(Deserialize)]
pub struct EconomicDataPoint {
    pub date: String,
    pub value: String,
}
