use crate::*;

pub trait Irrigator {
    fn start(&self);
    fn stop(&self);
    fn name(&self) -> &str;

    fn flow(&self) -> &FlowInfo;
    fn flow_mut(&mut self) -> &mut FlowInfo;

    fn settings(&self) -> StationSettings {
        Settings::station(self.name())
    }

    fn is_broken(&self) -> bool {
        self.flow().pressure > self.settings().pressure_threshold
    }
}

pub struct IrrigationController<I>
where
    I: Irrigator,
{
    pub stations: Vec<I>,
}

impl<I> IrrigationController<I>
where
    I: Irrigator,
{
    pub fn new() -> Self {
        Self {
            stations: Vec::new(),
        }
    }

    pub fn stations_mut(&mut self) -> &mut [I] {
        &mut self.stations
    }

    pub fn run_irrigation(&self) {
        for station in &self.stations {
            if !station.is_broken() {
                self.run_station(station);
            }
        }
    }

    fn run_station(&self, station: &I) {
        let length = station.settings().irrigation_length;
        station.start();
        // wait
        station.stop();
    }
}

pub struct LiveIrrigation {
    pin_num: u8,
    name: String,
    flow: FlowInfo,
}

impl Irrigator for LiveIrrigation {
    fn start(&self) {}

    fn stop(&self) {}

    fn name(&self) -> &str {
        &self.name
    }

    fn flow(&self) -> &FlowInfo {
        &self.flow
    }

    fn flow_mut(&mut self) -> &mut FlowInfo {
        &mut self.flow
    }
}

#[cfg(test)]
mod test {
    // use super::*;
}
