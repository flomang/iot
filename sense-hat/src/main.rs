mod led;
mod sensors;

use led::screen::LedControls;
use sensors::atmospheric::Atmospheric;

use log::{debug, info};
use sensehat::SenseHat;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut sensor = Atmospheric::new();
    let temp = sensor.get_temperature();
    info!("Temp: {:?}", temp);

    let mut led = LedControls::new();
    led.scroll_text("hello world");
    led.display_symbol(&led::HALLOWEEN, 3);

    info!("done!");
}
