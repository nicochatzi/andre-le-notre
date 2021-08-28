use crate::*;
use chrono::Utc;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Default, Debug)]
pub struct RainInfo {
    pub amount: f64,
    pub from: i64,
    pub to: i64,
}

pub trait RainDetector {
    fn read() -> RainInfo;
}

pub struct RainDetectorController<RD>
where
    RD: RainDetector,
{
    pub readings: HashMap<i64, RainInfo>,
    _detector: PhantomData<RD>,
}

impl<RD> RainDetectorController<RD>
where
    RD: RainDetector,
{
    pub fn new() -> Self {
        Self {
            readings: HashMap::new(),
            _detector: PhantomData,
        }
    }

    pub fn update(&mut self) {}
}

fn now() -> i64 {
    Utc::now().timestamp()
}

pub struct LiveRainDetector;

impl RainDetector for LiveRainDetector {
    fn read() -> RainInfo {
        RainInfo::default()
    }
}

#[cfg(test)]
mod test {
    // use super::*;
}
