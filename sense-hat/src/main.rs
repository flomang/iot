mod led;
mod sensors;

use led::screen::LedControls;
use sensors::atmospheric::Atmospheric;

use log::{debug, info};
use sensehat::SenseHat;
use sensehat_stick::{JoyStick, JoyStickEvent, Action, Direction};

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let mut sensor = Atmospheric::new();
    let temp = sensor.get_temperature();
    info!("Temp: {:?}", temp);

    let mut led = LedControls::new();
    led.scroll_text("hello world");
    led.display_symbol(&led::HALLOWEEN, 3);

    let mut stick = JoyStick::open().unwrap();
    loop {
        for ev in &stick.events().unwrap() {
            info!("Stick -- {:?}", ev);
        }
    }
}
