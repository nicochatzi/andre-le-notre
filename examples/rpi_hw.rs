use rppal::gpio::{Gpio, InputPin, Level, OutputPin};
use std::{
    error::Error,
    thread,
    time::{Duration, Instant},
};

const FLOW_METER_READING_PERIOD_MS: u64 = 5;

const FLOW_METER_PIN: u8 = 0;
const RAIN_METER_PIN: u8 = 0;
const STATION_PINS: [u8; 4] = [0, 0, 0, 0];

struct Meters {
    flow_meter: InputPin,
    rain_meter: InputPin,
}

impl Meters {
    fn new(gpio: Gpio) -> Self {
        Self {
            flow_meter: gpio.get(FLOW_METER_PIN)?.into_input(),
            rain_meter: gpio.get(RAIN_METER_PIN)?.into_input(),
        }
    }

    fn check_flow_meter(&self) -> f32 {
        self.check_meter(&self.flow_meter)
    }

    fn check_rain_meter(&self) -> f32 {
        self.check_meter(&self.rain_meter)
    }

    fn check_meter(&self, meter: &InputPin) -> f32 {
        let start = Instant::now();

        let mut ticks = 100_000;
        let mut num_switches = 0;
        let mut previous_reading = Level::Low;
        while ticks != 0 {
            let reading = meter.read();
            if previous_reading != reading {
                num_switches += 1;
            }
            ticks -= 1;
        }

        let duration_ns = start.elapsed().as_nanos();
        let avg_tick_time_ns = duration_ns as f32 / num_switches as f32;
        avg_tick_time_ns * 2.0 / 1_000_000_000.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let gpio = Gpio::new()?;
    let mut stations = STATION_PINS
        .iter()
        .map(|pin| gpio.get(pin)?.into_output())
        .collect();
    let mut andre = Meters::new(gpio);

    println!("rain avg : {}", meters.check_rain_meter());
    println!("flow avg : {}", meters.check_flow_meter());

    for i in STATION_PINS.len() {
        stations[i].set_high();
        thread::sleep(Duration::from_secs(5));

        println!("flow avg : {}", meters.check_flow_meter());

        stations[i].set_low();
        thread::sleep(Duration::from_secs(5));
    }

    Ok(())
}
