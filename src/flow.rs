use crate::*;
use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct FlowInfo {
    pub pressure: f32,
}

pub trait FlowMeter {
    fn read() -> FlowInfo;
}

pub struct FlowMeterController<FM, S>
where
    FM: FlowMeter,
    S: Irrigator,
{
    _flow: PhantomData<FM>,
    _station: PhantomData<S>,
}

impl<FM, S> FlowMeterController<FM, S>
where
    FM: FlowMeter,
    S: Irrigator,
{
    pub fn new() -> Self {
        Self {
            _flow: PhantomData,
            _station: PhantomData,
        }
    }

    pub fn update_stations(&mut self, stations: &mut [S]) {
        for station in stations {
            if !station.is_broken() {
                *station.flow_mut() = self.run_station_check(station);
            }
        }
    }

    fn run_station_check(&self, station: &S) -> FlowInfo {
        FlowInfo::default()
    }
}

pub struct LiveFlowMeter;

impl FlowMeter for LiveFlowMeter {
    fn read() -> FlowInfo {
        FlowInfo::default()
    }
}

#[cfg(test)]
mod test {
    // use super::*;
}
