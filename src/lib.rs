mod flow;
mod irrigate;
mod rain;
mod settings;

use flow::*;
use irrigate::*;
use rain::*;
use settings::*;

fn sleep_ms(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

pub fn run_weather_station() {
    let mut rain_check_controller = RainDetectorController::<LiveRainDetector>::new();

    loop {
        rain_check_controller.update();
        sleep_ms(10);
    }
}

pub fn run_irrigation_system() {
    let mut flow_controller = FlowMeterController::<LiveFlowMeter, LiveIrrigation>::new();
    let mut irrigation_controller = IrrigationController::<LiveIrrigation>::new();

    loop {
        flow_controller.update_stations(irrigation_controller.stations_mut());
        irrigation_controller.run_irrigation();

        sleep_ms(60 * 60 * 1000);
    }
}
