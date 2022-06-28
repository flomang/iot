mod led;

use led::screen::LedControls;

fn main() {
    let mut led = LedControls::new();

    led.scroll_text("hello world");
    led.display_symbol(&led::HALLOWEEN, 12);

    println!("done!");
}
