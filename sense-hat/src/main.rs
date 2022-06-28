mod led;

use led::screen::LedControls;
use sensehat::SenseHat;

fn main() {
    let mut led = LedControls::new();

    led.scroll_text("hello world");
    led.display_symbol(&led::HALLOWEEN, 12);

    let mut hat = SenseHat::new().unwrap();
    let temp = hat.get_temperature_from_humidity().unwrap().as_celsius();


    println!("Temp: {:?} C", temp);
}
