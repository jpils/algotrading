use crate::{prelude::*, types::{Hours, Interval, Minutes, Seconds, TimeUnit}};
use chrono::{DateTime, Utc};
use crate::data::{MarketData, OHLCV};

pub trait DataFetcher<T: MarketData> {
    type Request: FetchRequest;

    fn fetch(&self, request: &Self::Request) -> impl Future<Output = Result<Vec<T>>> + Send;
}

pub trait FetchRequest {}

pub struct OHLCVRequest {
    pub symbol: String,
    pub interval: Interval,
    pub limit: u32,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>
}

impl FetchRequest for OHLCVRequest {
    
}

pub mod binance;
