use chrono::{DateTime, Utc};
use crate::prelude::*;

pub trait MarketData {}

pub struct OHLCV {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>
}

impl MarketData for OHLCV {}

impl OHLCV {
    pub fn new(
        open: f64,
        high: f64,
        low: f64,
        close: f64,
        volume: f64,
        open_time: DateTime<Utc>,
        close_time: DateTime<Utc>
    ) -> Result<Self> {

        if high < low {
            return Err(Error::Generic("high is smaller than low!".into()));
        }
        if close_time < open_time {
            return Err(Error::Generic("start_time is later than end_time!".into()));
        }
        if !volume.is_normal() || volume < 0.0 {
            return Err(Error::Generic(format!("volume is not a valid number! Got {volume}")));
        }

        Ok(Self {
            open,
            high,
            low,
            close,
            volume,
            start_time: open_time,
            end_time: close_time
        })
    }

    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn open_time(&self) -> DateTime<Utc> {
        self.start_time
    }

    pub fn close_time(&self) -> DateTime<Utc> {
        self.end_time
    }
}

#[cfg(test)]
mod tests;
