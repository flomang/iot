/**
 * This is our daily manager runner.
 */
//use tokio::prelude::*;
use tokio::time::{interval_at, Duration, Instant};

use crate::manager::{Action, Display, Tx};
use chrono::prelude::*;
use log::{debug, info};

// tag::run[]
const INTERVAL_IN_SECONDS: u64 = 60 * 60;

pub fn run(mut tx: Tx) {
    use std::ops::Add;

    let local: DateTime<Local> = Local::now(); // <1>
    let min = local.minute();

    // Determine the time till the top of the hour
    let time_from_hour = 60 - min; 
    debug!("Min from hour : {:?}", time_from_hour);
    let time_at_hour = Instant::now();
    time_at_hour.add(Duration::from_secs((60 * time_from_hour).into())); 

    // Compute the interval
    let mut interval = interval_at(time_at_hour, Duration::from_secs(INTERVAL_IN_SECONDS)); 
    tokio::spawn(async move {
        // run on initial start up then timers after
        run_initial(&mut tx).await; 

        loop {
            interval.tick().await; 
            info!("Daily special");
            display_special(&mut tx); 
        }
    });
}

async fn send(tx: &mut Tx, action: Action) {
    if let Err(_) = tx.send(action).await {
        info!("receiver dropped");
        return;
    }
}
// end::run[]

/**
 * Run on initial start.
 */
// tag::initial[]
async fn run_initial(tx: &mut Tx) {
    let local: DateTime<Local> = Local::now();
    if is_christmas(&local) {
        send(tx, Action::Print(Display::Christmas)).await;
    } else if is_halloween(&local) {
        send(tx, Action::Print(Display::Halloween)).await;
    } else if is_valentines(&local) {
        send(tx, Action::Print(Display::Valentines)).await;
    } else {
        send(tx, Action::Print(Display::Text("OK".to_string()))).await;
    }
}
// end::initial[]

/**
 * Calculate the daily special that we should send.
 */
async fn display_special(tx: &mut Tx) {
    let local: DateTime<Local> = Local::now();

    // now switch based o the variable to display
    // we will only call this on the hour so we don't need to check the minute
    // also could be a delay so better to not be that precise
    if local.hour() == 8 {
        //display_weather(tx);
        send(tx, Action::ShowTemperature).await;
    } else if local.hour() == 12 {
        if is_christmas(&local) {
            send(tx, Action::Print(Display::Christmas)).await;
        } else if is_halloween(&local) {
            send(tx, Action::Print(Display::Halloween)).await;
        } else {
            send(tx, Action::Print(Display::Valentines)).await;
        }
    }
}

fn is_halloween(local: &DateTime<Local>) -> bool {
    // any day in October 
    local.month() == 10
}

fn is_christmas(local: &DateTime<Local>) -> bool {
    // any day in December before or on the 25th
    local.month() == 12 && local.day() <= 25
}

fn is_valentines(local: &DateTime<Local>) -> bool {
    // any day in December before or on the 25th
    local.month() == 2
}
