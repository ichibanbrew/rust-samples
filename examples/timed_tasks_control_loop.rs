use chrono::{Timelike, Utc};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keep_running = Arc::new(AtomicBool::new(false));
    let loop_keep_running = keep_running.clone();

    tokio::spawn(async move {
        loop {
            let now = Utc::now();
            let hour = now.hour();
            let minute = now.minute();
            let second = now.second();
            // Run in the first 10 minutes of every hour
            if minute < 10 {
                keep_running.store(true, Ordering::Relaxed);
                println!("Control loop: awake");
            } else {
                keep_running.store(false, Ordering::Relaxed);
                println!(
                    "Control loop: The current UTC time is {:02}:{:02}:{:02}. Sleeping...",
                    hour, minute, second,
                );
            }
            time::sleep(time::Duration::from_secs(30)).await;
        }
    });

    loop {
        if !loop_keep_running.load(Ordering::Relaxed) {
            println!("App loop: not running");
            time::sleep(time::Duration::from_secs(10)).await;
            continue;
        }
        println!("App loop: running");

        let task_1_keep_running = loop_keep_running.clone();
        let task_2_keep_running = loop_keep_running.clone();

        let task_1 = tokio::spawn(async move {
            while task_1_keep_running.load(Ordering::Relaxed) {
                time::sleep(time::Duration::from_secs(10)).await;
                println!("Task 1 woke up :)");
            }
        });

        let task_2 = tokio::spawn(async move {
            while task_2_keep_running.load(Ordering::Relaxed) {
                time::sleep(time::Duration::from_secs(10)).await;
                println!("Task 2 woke up and is doing something");
            }
        });

        task_1.await.unwrap();
        task_2.await.unwrap();
    }
}
