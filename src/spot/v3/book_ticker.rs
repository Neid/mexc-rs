use crate::spot::v3::{ApiResponse, ApiResult};
use crate::spot::MexcSpotApiTrait;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerParams<'a> {
    /// Symbol
    pub symbol: &'a str,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerOutput {
    pub symbol: String,
    pub bid_price: Decimal,
    pub bid_qty: Decimal,
    pub ask_price: Decimal,
    pub ask_qty: Decimal,
}

#[async_trait]
pub trait BookTickerEndpoint {
    /// Order book
    async fn book_ticker(&self, params: BookTickerParams<'_>) -> ApiResult<BookTickerOutput>;
}

#[async_trait]
impl<T: MexcSpotApiTrait + Sync> BookTickerEndpoint for T {
    async fn book_ticker(&self, params: BookTickerParams<'_>) -> ApiResult<BookTickerOutput> {
        let endpoint = format!("{}/api/v3/ticker/bookTicker", self.endpoint().as_ref());
        let response = self
            .reqwest_client()
            .get(&endpoint)
            .query(&params)
            .send()
            .await?;
        let api_response = response.json::<ApiResponse<BookTickerOutput>>().await?;
        let output = api_response.into_api_result()?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::spot::MexcSpotApiClient;

    use super::*;

    #[tokio::test]
    async fn test_book_ticker() {
        let client = MexcSpotApiClient::default();
        let avg_params = BookTickerParams { symbol: "BTCUSDT" };
        let result = client.book_ticker(avg_params).await;
        assert!(result.is_ok());
    }
}
