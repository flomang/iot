#![allow(dead_code, unused_must_use)]
extern crate error_chain;

mod daily;
mod errors;
mod joystick;
mod led;
mod manager;
mod sensors;

use errors::*;
use led::screen::LedControls;
use sensors::atmospheric::Atmospheric;

use log::{debug, info};
use manager::Tx;
use std::result::Result;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
struct Application {
    backend: String,
    uuid: String,
}

// TODO move this to a util module
const UUID_LOCATION: &str = "/var/uuid";
fn read_device_id() -> MyResult<Uuid> {
    use std::fs;

    let uuid_str = fs::read_to_string(UUID_LOCATION.to_string())?; // <1>

    let uuid = Uuid::parse_str(uuid_str.trim()).unwrap(); // <2>

    debug!("Device UUID :: {:?}", uuid);
    Ok(uuid)
}

fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let uuid = read_device_id()
        .chain_err(|| "No device id file found")
        .unwrap()
        .to_string();
    //let mut sensor = Atmospheric::new();
    //let temp = sensor.get_temperature();
    //info!("Temp: {:?}", temp);

    //let mut led = LedControls::new();
    //led.scroll_text("hello world");
    //led.display_symbol(&led::HALLOWEEN, 3);

    //let mut stick = JoyStick::open().unwrap();
    //loop {
    //    for ev in &stick.events().unwrap() {
    //        info!("Stick -- {:?}", ev);
    //    }
    //}

    let app = Application {
        backend: "RASP-MO-6".to_string(),
        uuid,
    };

    run(app.clone());
}

#[tokio::main]
//async fn run(backend: &'static str, uuid: &'static str) -> Result<(), Box<dyn std::error::Error>> {
async fn run(_app: Application) -> Result<(), Box<dyn std::error::Error>> {
    use tokio::sync::{mpsc, watch};

    info!("Setup and start our channel runners - full ...");

    // for multi producer, multi consumer
    let (tx, rx) = mpsc::channel(100);
    let joy_tx: Tx = tx.clone();
    let daily_tx: Tx = tx.clone();
    let _temp_cmd_tx: Tx = tx.clone();
    // tag::face[]
    let _motion_cmd_tx: Tx = tx.clone();

    // basically like a watcher but we needed features that watcher didnt provide
    // Single producer/ consumer for this setup
    let (_face_tx, face_rx) = mpsc::channel(1);

    // Face detector, for single producer single consumer
    // One shot can not be used in loops its deisgned for one shot
    // let (motion_tx, motion_rx) = oneshot::channel::<bool>();
    let (motion_tx, _motion_rx) = watch::channel(false);
    // end::face[]

    // Temp Detection, for single producer, multi consumer
    // channel sets the initial value, we will set to room temperature
    let (temp_tx, mut _temp_rx) = watch::channel(25f32);

    // Start our timer matcher
    // we want to do this after the authentication so we don't have any interruption from the
    // login; this will also run Asynchronously
    daily::run(daily_tx);

    // Setup and run the Joystick now;
    joystick::run(joy_tx);

    // Captures the video camera
    //camera::run_video_capture(face_tx);

    // send the camera recording on an hourly basis
    //camera::hourly_upload(app.uuid, app.backend);

    // TODO : Send temperature to file
    // Run our home kit setup
    //homekit::initialize(motion_cmd_tx, motion_rx, temp_cmd_tx, temp_rx);

    // Ready our receivers
    let led_controls = Arc::new(Mutex::new(LedControls::new()));
    let atmospheric = Arc::new(Mutex::new(Atmospheric::new()));

    manager::run(rx, temp_tx, face_rx, motion_tx, &led_controls, &atmospheric).await;

    debug!("Complete");
    Ok(())
}
