use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;

/// Request parameters for the price v1 API
#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceV1Request {
    /// Token symbols or mint addresses to get prices for (comma-separated)
    pub ids: String,
    /// Optional token to price against (defaults to USDC if not provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vs_token: Option<String>,
}

/// Price information for a single token in v1
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TokenV1Price {
    /// The mint address of the token
    pub id: String,
    /// Symbol of the token (if available)
    #[serde(default)]
    pub mint_symbol: String,
    /// The mint address of the vs token
    pub vs_token: String,
    /// Symbol of the vs token (if available)
    pub vs_token_symbol: String,
    /// The price of the token (vs USDC or the specified vsToken)
    pub price: f64,
}

/// Response from the price v1 API
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceV1Response {
    /// Map of token mint address or symbol to token price information
    pub data: HashMap<String, TokenV1Price>,
    /// Time taken for the request to complete
    pub time_taken: f64,
}

/// Error response for the price v1 API
#[derive(Deserialize, Debug, Clone)]
pub struct PriceV1Error {
    pub error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl PriceV1Request {
    /// Create a new price request for a single token
    pub fn new_single(token_id: &str) -> Self {
        Self {
            ids: token_id.to_string(),
            vs_token: None,
        }
    }

    /// Create a new price request for a single token using Pubkey
    pub fn new_single_pubkey(token_mint: &Pubkey) -> Self {
        Self {
            ids: token_mint.to_string(),
            vs_token: None,
        }
    }

    /// Create a new price request for multiple tokens
    pub fn new_multiple(token_ids: &[&str]) -> Self {
        let ids = token_ids.join(",");

        Self {
            ids,
            vs_token: None,
        }
    }

    /// Create a new price request for multiple tokens using Pubkeys
    pub fn new_multiple_pubkeys(token_mints: &[Pubkey]) -> Self {
        let ids = token_mints
            .iter()
            .map(|mint| mint.to_string())
            .collect::<Vec<String>>()
            .join(",");

        Self {
            ids,
            vs_token: None,
        }
    }

    /// Add a vs token to the request
    pub fn with_vs_token(mut self, vs_token: &str) -> Self {
        self.vs_token = Some(vs_token.to_string());
        self
    }

    /// Add a vs token to the request using Pubkey
    pub fn with_vs_token_pubkey(mut self, vs_token: &Pubkey) -> Self {
        self.vs_token = Some(vs_token.to_string());
        self
    }
}