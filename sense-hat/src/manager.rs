/// This will handle our asynchronous running of our applications.
/// 
/// 
use tokio::sync::{mpsc, watch};

use log::info;

use std::sync::{Arc, Mutex};

use crate::led::screen::LedControls;
use crate::led::{HALLOWEEN, CHRISTMAS_TREE, HEART};
use crate::sensors::atmospheric::Atmospheric;

// tag::txrx[]
pub type Tx = mpsc::Sender<Action>;     // <1>
pub type Rx = mpsc::Receiver<Action>;
// end::txrx[]

pub type FaceTx = mpsc::Sender<bool>;
pub type FaceRx = mpsc::Receiver<bool>;

pub type MotionTx = watch::Sender<bool>;
pub type MotionRx = watch::Receiver<bool>;

pub type TempTx = watch::Sender<f32>;
pub type TempRx = watch::Receiver<f32>;

pub async fn run(   mut rx: Rx,
                    temp_tx: TempTx,
                    mut face_rx: FaceRx,
                    motion_tx: MotionTx,
                    led_controls: &Arc<Mutex<LedControls>>,
                    atmospheric: &Arc<Mutex<Atmospheric>>) {
    // tag::motion_set[]
    let motion = Arc::new(Mutex::new(false)); // <1>
    let motion_set = motion.clone();

    // Receives the motion
    // Spawn a task to manage the counter
    tokio::spawn(async move {
        while let Some(movement) = face_rx.recv().await {
            let mut m = motion_set.lock().unwrap(); // <2>
            *m = movement;
        }
    });
    // end::motion_set[]

    // Receives the information
    while let Some(action) = rx.recv().await {
        info!("Received :: {:?}", action);
        // now lets parse out what should happen.Action
        match action {
            Action::ShowTemperature => {
                display_weather(&atmospheric, &led_controls);
            },
            Action::Print(display) => {
                match display {
                    Display::Halloween => {
                        display_halloween(&led_controls);
                    },
                    Display::Christmas => {
                        display_christmas(&led_controls);
                    },
                    Display::Valentines => {
                        display_valentines(&led_controls);
                    },
                    Display::Text(text) => {
                        display_text(text, &led_controls);
                    },
                    Display::QuestionMark => {
                        question_mark(&led_controls);
                    }
                }
            },
            // tag::temp_action[]
            Action::SendTemperature => {
                send_temperature(&atmospheric, &temp_tx)
            },
            // end::temp_action[]
            // tag::send_motion[]
            Action::SendMotion => {
                send_motion(&motion, &motion_tx);
            }
            // end::send_motion[]
        }
        // you could potentially blank it after a few seconds
        // This is left for the reader.
    }
}

#[derive(Debug)]
pub enum Action {
    ShowTemperature,
    Print(Display),
    SendTemperature,
    SendMotion
}

#[derive(Debug)]
pub enum Display {
    Halloween,
    Christmas,
    QuestionMark,
    Text(String),
    Valentines
}

fn send_motion(motion: &Arc<Mutex<bool>>, tx: &MotionTx) {
    let m = motion.lock().unwrap();
    tx.send(*m);
}

fn send_temperature(atmospheric: &Arc<Mutex<Atmospheric>>, temp_tx: &TempTx) {
    let mut atmo = atmospheric.lock().unwrap();
    let temp = atmo.get_temperature_in_c();
    temp_tx.send(temp);
}

fn question_mark(led_controls: &Arc<Mutex<LedControls>>) {
    //let mut led = Arc::get_mut(&mut led_controls).unwrap();    
    let mut led = led_controls.lock().unwrap();
    led.question();
}

// Display christmas tree for 30 seconds
fn display_christmas(led_controls: &Arc<Mutex<LedControls>>) {
    let mut led = led_controls.lock().unwrap();
    led.display_symbol(&CHRISTMAS_TREE, 0);
}

fn display_valentines(led_controls: &Arc<Mutex<LedControls>>) {
    let mut led = led_controls.lock().unwrap();
    led.display_symbol(&HEART, 0);
}

 // Display pumpkin tree for 30 seconds
fn display_halloween(led_controls: &Arc<Mutex<LedControls>>) {
    let mut led = led_controls.lock().unwrap();
    led.display_symbol(&HALLOWEEN, 0);
}

fn display_weather(atmospheric: &Arc<Mutex<Atmospheric>>, led_controls: &Arc<Mutex<LedControls>>) {
    let mut atmo = atmospheric.lock().unwrap();
    let temp: String = atmo.get_temperature();
    let mut led = led_controls.lock().unwrap();
    led.display(&temp);
}

// Display any text
fn display_text(text: String, led_controls: &Arc<Mutex<LedControls>>) {
    let mut led = led_controls.lock().unwrap();
    led.scroll_text(&text);
}