use crate::models::{AssetClass, InvestmentDecision};
use std::collections::HashMap;

pub fn create_asset_classes() -> Vec<AssetClass> {
    vec![
        AssetClass {
            name: String::from("US Equities"),
            expected_return: 0.07,
            volatility: 0.15,
            correlation: HashMap::from([
                (String::from("US Bonds"), -0.2),
                (String::from("International Equities"), 0.8),
                (String::from("Commodities"), 0.3),
                (String::from("Real Estate"), 0.6),
            ]),
        },
        AssetClass {
            name: String::from("US Bonds"),
            expected_return: 0.03,
            volatility: 0.05,
            correlation: HashMap::from([
                (String::from("US Equities"), -0.2),
                (String::from("International Equities"), -0.1),
                (String::from("Commodities"), 0.0),
                (String::from("Real Estate"), 0.2),
            ]),
        },
        AssetClass {
            name: String::from("International Equities"),
            expected_return: 0.08,
            volatility: 0.18,
            correlation: HashMap::from([
                (String::from("US Equities"), 0.8),
                (String::from("US Bonds"), -0.1),
                (String::from("Commodities"), 0.4),
                (String::from("Real Estate"), 0.5),
            ]),
        },
        AssetClass {
            name: String::from("Commodities"),
            expected_return: 0.05,
            volatility: 0.20,
            correlation: HashMap::from([
                (String::from("US Equities"), 0.3),
                (String::from("US Bonds"), 0.0),
                (String::from("International Equities"), 0.4),
                (String::from("Real Estate"), 0.2),
            ]),
        },
        AssetClass {
            name: String::from("Real Estate"),
            expected_return: 0.06,
            volatility: 0.12,
            correlation: HashMap::from([
                (String::from("US Equities"), 0.6),
                (String::from("US Bonds"), 0.2),
                (String::from("International Equities"), 0.5),
                (String::from("Commodities"), 0.2),
            ]),
        },
    ]
}

pub fn calculate_risk_parity(asset_classes: &[AssetClass]) -> Vec<InvestmentDecision> {
    let total_inverse_volatility: f64 = asset_classes.iter().map(|ac| 1.0 / ac.volatility).sum();

    asset_classes
        .iter()
        .map(|ac| {
            let allocation = (1.0 / ac.volatility) / total_inverse_volatility;
            InvestmentDecision {
                asset: ac.name.clone(),
                allocation,
            }
        })
        .collect()
}

pub fn adjust_allocations(
    base_allocations: Vec<InvestmentDecision>,
    environment: &HashMap<String, f64>,
) -> Vec<InvestmentDecision> {
    let mut adjusted_allocations = base_allocations;
    let gdp_growth = environment.get("GDP Growth").unwrap_or(&2.0);
    let inflation_rate = environment.get("Inflation Rate").unwrap_or(&2.0);

    for decision in &mut adjusted_allocations {
        match decision.asset.as_str() {
            "US Equities" | "International Equities" => {
                decision.allocation *= 1.0 + (gdp_growth - 2.0) * 0.1;
            }
            "US Bonds" => {
                decision.allocation *= 1.0 - (inflation_rate - 2.0) * 0.1;
            }
            "Commodities" => {
                decision.allocation *= 1.0 + (inflation_rate - 2.0) * 0.1;
            }
            _ => {}
        }
    }

    // Normalize allocations
    let total_allocation: f64 = adjusted_allocations.iter().map(|d| d.allocation).sum();
    for decision in &mut adjusted_allocations {
        decision.allocation /= total_allocation;
    }

    adjusted_allocations
}
