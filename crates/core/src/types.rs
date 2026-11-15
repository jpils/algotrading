use std::fmt::Display;

pub trait TimeUnit {
    fn as_u32(&self) -> u32;
}

pub struct Seconds(u32);

impl TimeUnit for Seconds {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Seconds {
    pub fn new(t: u32) -> Self {
        Self(t)
    }
}

pub struct Minutes(u32);

impl TimeUnit for Minutes {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Minutes {
    pub fn new(t: u32) -> Self {
        Self(t)
    }
}

pub struct Hours(u32);

impl TimeUnit for Hours {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

impl Hours {
    pub fn new(t: u32) -> Self {
        Self(t)
    }
}

pub enum Interval {
    Seconds(Seconds),
    Minutes(Minutes),
    Hours(Hours)
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seconds(t) => write!(f, "{}s", t.as_u32()),
            Self::Minutes(t) => write!(f, "{}m", t.as_u32()),
            Self::Hours(t) => write!(f, "{}h", t.as_u32()),
        }
    }
}

impl Interval {
    fn as_u32(&self) -> u32 {
        match self {
            Self::Seconds(t) => t.as_u32(),
            Self::Minutes(t) => t.as_u32(),
            Self::Hours(t) => t.as_u32()
        }
    }
}
