use std::fmt::Debug;

use chrono::DateTime;
use serde::Deserialize;
use serde::de::IgnoredAny;

use crate::fetcher::OHLCVRequest;
use crate::prelude::*;
use crate::{data::OHLCV, fetcher::DataFetcher};

pub struct BinanceFetcher {
    client: reqwest::Client,
}

impl DataFetcher<OHLCV> for BinanceFetcher {
    type Request = OHLCVRequest;

    async fn fetch(&self, request: &Self::Request) -> Result<Vec<OHLCV>> {
        let query = [
            ("symbol" , &request.symbol),
            ("interval", &request.interval.to_string()),
            ("limit", &request.limit.to_string())
        ];

        let response = self.client
            .get("https://data-api.binance.vision/api/v3/klines")
            .query(&query)
            .send()
            .await?
            .error_for_status()?;

        let candles: Vec<BinanceKline> = response.json().await?;

        let candles: Vec<OHLCV> = candles.into_iter()
            .map(|candle| candle.try_into())
            .collect::<Result<Vec<_>>>()?;

        Ok(candles)
    } 
}

impl BinanceFetcher {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self { client }
    }
}

#[derive(Deserialize, Debug)]
struct BinanceKline(
    i64,
    String,
    String,
    String,
    String,
    String,
    i64,
    IgnoredAny,
    IgnoredAny,
    IgnoredAny,
    IgnoredAny,
    IgnoredAny,
);

impl TryFrom<BinanceKline> for OHLCV {
    type Error = Error;
    
    fn try_from(value: BinanceKline) -> Result<Self> {
        let BinanceKline(
            open_time, 
            open, 
            high, 
            low, 
            close, 
            volume, 
            close_time, 
            ..
        ) = value;

        let open_time = DateTime::from_timestamp_millis(open_time)
            .ok_or_else(|| Error::Generic("Milliseconds out of range!".into()))?;

        let close_time = DateTime::from_timestamp_millis(close_time)
            .ok_or_else(|| Error::Generic("Milliseconds out of range!".into()))?;

        OHLCV::new(
            open.parse()?, 
            high.parse()?, 
            low.parse()?, 
            close.parse()?, 
            volume.parse()?, 
            open_time,  
            close_time
        )
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

use crate::types::Hours;

use super::*;

    #[tokio::test]
    async fn test_binance_fetch_live() {
        let request = OHLCVRequest{
            symbol: "BTCUSDC".into(),
            interval: crate::types::Interval::Hours(Hours::new(1)),
            limit: 100,
            start: Utc.with_ymd_and_hms(2026, 11, 1, 0, 0, 0).single().unwrap(),
            end: Utc.with_ymd_and_hms(2026, 11, 2, 0, 0, 0).single().unwrap()

        };

        let fetcher = BinanceFetcher::new();

        let candles: Vec<OHLCV> = fetcher.fetch(&request).await.unwrap();

        assert!(!candles.is_empty());
    }

    #[test]
    fn test_binance_fetch_parsing() {
        let json = r#"[
            1704067200000,
            "100.0",
            "110.0",
            "90.0",
            "105.0",
            "1234.5",
            1704070799999,
            "0",
            0,
            "0",
            "0",
            "0"
        ]"#;

        let kline: BinanceKline = serde_json::from_str(json).unwrap();
        let candle = OHLCV::try_from(kline).unwrap();

        assert_eq!(candle.open(), 100.0);
        assert_eq!(candle.high(), 110.0);
        assert_eq!(candle.low(), 90.0);
        assert_eq!(candle.close(), 105.0);
        assert_eq!(candle.volume(), 1234.5);
    }
    
    #[test]
    fn test_rate_limit_error() {
        todo!()
    }
}
