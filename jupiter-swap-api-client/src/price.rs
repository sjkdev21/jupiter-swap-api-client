use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;


/// Helper module for serializing/deserializing Option<u64> as string
pub mod field_as_string_option {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => serializer.serialize_str(&v.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => s
                .parse()
                .map(Some)
                .map_err(|_| serde::de::Error::custom("Failed to parse u64")),
            None => Ok(None),
        }
    }
}

/// Request parameters for the price API
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceRequest {
    /// Comma-separated list of token mint addresses to get prices for
    pub ids: String,
    /// Optional vs token to price against (defaults to USDC if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs_token: Option<String>,
    /// Include extra information about the price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_extra_info: Option<bool>,
}

/// Price information for a single token
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenPrice {
    /// The mint address of the token
    pub id: String,
    /// The type of price (usually "derivedPrice")
    pub r#type: String,
    /// The price of the token (vs USDC or the specified vsToken)
    pub price: String,
    /// Extra information about the price (only present if showExtraInfo=true)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<PriceExtraInfo>,
}

/// Last swapped price information
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastSwappedPrice {
    /// Epoch seconds of the last Jupiter sell price
    pub last_jupiter_sell_at: Option<u64>,
    /// Price of last Jupiter sell
    pub last_jupiter_sell_price: Option<String>,
    /// Epoch seconds of the last Jupiter buy price
    pub last_jupiter_buy_at: Option<u64>,
    /// Price of last Jupiter buy
    pub last_jupiter_buy_price: Option<String>,
}

/// Quoted price information
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuotedPrice {
    /// The quoted buy price
    pub buy_price: Option<String>,
    /// Epoch seconds of when the buy quote was retrieved
    pub buy_at: Option<u64>,
    /// The quoted sell price
    pub sell_price: Option<String>,
    /// Epoch seconds of when the sell quote was retrieved
    pub sell_at: Option<u64>,
}

/// Price impact ratios for different depths
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceImpactRatio {
    /// Map of depth levels (10, 100, 1000 SOL) to impact percentages
    pub depth: HashMap<String, f64>,
    /// Timestamp of when the depth data was collected
    pub timestamp: u64,
}

/// Depth information for price impacts
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthInfo {
    /// Price impact ratio for buy operations
    pub buy_price_impact_ratio: Option<PriceImpactRatio>,
    /// Price impact ratio for sell operations
    pub sell_price_impact_ratio: Option<PriceImpactRatio>,
}

/// Extra information about a token's price
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceExtraInfo {
    /// Information about the last swapped price
    pub last_swapped_price: Option<LastSwappedPrice>,
    /// Information about the quoted price
    pub quoted_price: Option<QuotedPrice>,
    /// Confidence level of the price (high, medium, or low)
    pub confidence_level: Option<String>,
    /// Depth information for various trade sizes
    pub depth: Option<DepthInfo>,
}

/// Response from the price API
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceResponse {
    /// Map of token mint address to token price information
    pub data: HashMap<String, TokenPrice>,
    /// Time taken for the request to complete
    pub time_taken: f64,
}

impl PriceRequest {
    /// Create a new price request for a single token
    pub fn new_single(token_mint: &Pubkey) -> Self {
        Self {
            ids: token_mint.to_string(),
            vs_token: None,
            show_extra_info: None,
        }
    }

    /// Create a new price request for multiple tokens
    pub fn new_multiple(token_mints: &[Pubkey]) -> Self {
        let ids = token_mints
            .iter()
            .map(|mint| mint.to_string())
            .collect::<Vec<String>>()
            .join(",");

        Self {
            ids,
            vs_token: None,
            show_extra_info: None,
        }
    }

    /// Add a vs token to the request
    pub fn with_vs_token(mut self, vs_token: &Pubkey) -> Self {
        self.vs_token = Some(vs_token.to_string());
        self
    }

    /// Include extra information in the response
    pub fn with_extra_info(mut self, show_extra_info: bool) -> Self {
        self.show_extra_info = Some(show_extra_info);
        self
    }
}