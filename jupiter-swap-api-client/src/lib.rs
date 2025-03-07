use std::collections::HashMap;

use quote::{InternalQuoteRequest, QuoteRequest, QuoteResponse};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use solana_sdk::pubkey::Pubkey;
use swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use thiserror::Error;
use price::{PriceRequest, PriceResponse};

pub mod quote;
pub mod route_plan_with_metadata;
pub mod serde_helpers;
pub mod swap;
pub mod transaction_config;
pub mod price;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Request failed with status {status}: {body}")]
    RequestFailed {
        status: reqwest::StatusCode,
        body: String,
    },
    #[error("Failed to deserialize response: {0}")]
    DeserializationError(#[from] reqwest::Error),
}

async fn check_is_success(response: Response) -> Result<Response, ClientError> {
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(ClientError::RequestFailed { status, body });
    }
    Ok(response)
}

async fn check_status_code_and_deserialize<T: DeserializeOwned>(
    response: Response,
) -> Result<T, ClientError> {
    let response = check_is_success(response).await?;
    response
        .json::<T>()
        .await
        .map_err(ClientError::DeserializationError)
}

impl JupiterSwapApiClient {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse, ClientError> {
        let url = format!("{}/quote", self.base_path);
        let extra_args = quote_request.quote_args.clone();
        let internal_quote_request = InternalQuoteRequest::from(quote_request.clone());
        let response = Client::new()
            .get(url)
            .query(&internal_quote_request)
            .query(&extra_args)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap(
        &self,
        swap_request: &SwapRequest,
        extra_args: Option<HashMap<String, String>>,
    ) -> Result<SwapResponse, ClientError> {
        let response = Client::new()
            .post(format!("{}/swap", self.base_path))
            .query(&extra_args)
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }

    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> Result<SwapInstructionsResponse, ClientError> {
        let response = Client::new()
            .post(format!("{}/swap-instructions", self.base_path))
            .json(swap_request)
            .send()
            .await?;
        check_status_code_and_deserialize::<SwapInstructionsResponseInternal>(response)
            .await
            .map(Into::into)
    }
    
    /// Get prices for one or more tokens from the Jupiter Price API v2
    /// 
    /// By default, prices are in terms of USDC. Use the vs_token parameter to get prices in terms of another token.
    pub async fn get_prices(&self, price_request: &PriceRequest) -> Result<PriceResponse, ClientError> {
        let url = format!("{}/price/v2", self.base_path);
        let response = Client::new()
            .get(url)
            .query(&price_request)
            .send()
            .await?;
        check_status_code_and_deserialize(response).await
    }
    
    /// Helper method to get the price for a single token in terms of USDC
    pub async fn get_token_price(&self, token_mint: &Pubkey) -> Result<PriceResponse, ClientError> {
        let request = PriceRequest::new_single(token_mint);
        self.get_prices(&request).await
    }
    
    /// Helper method to get prices for multiple tokens in terms of USDC
    pub async fn get_token_prices(&self, token_mints: &[Pubkey]) -> Result<PriceResponse, ClientError> {
        let request = PriceRequest::new_multiple(token_mints);
        self.get_prices(&request).await
    }
    
    /// Helper method to get the price of a token in terms of another token
    pub async fn get_token_pair_price(
        &self,
        token_mint: &Pubkey,
        vs_token: &Pubkey
    ) -> Result<PriceResponse, ClientError> {
        let request = PriceRequest::new_single(token_mint).with_vs_token(vs_token);
        self.get_prices(&request).await
    }
    
    /// Helper method to get detailed price information including extra details
    pub async fn get_detailed_price(&self, token_mint: &Pubkey) -> Result<PriceResponse, ClientError> {
        let request = PriceRequest::new_single(token_mint).with_extra_info(true);
        self.get_prices(&request).await
    }
}