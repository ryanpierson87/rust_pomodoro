use std::time::Duration;
use std::thread;
use std::fmt::format;

pub fn countdown(minutes: i32, phase: &str) {
    // count_down(1);
    if phase == "work"{
        println!("Let's get down to business");
    } else if phase == "rest"{
        println!("Take a break! You've earned it");
    }
    let mut timer = minutes * 60;
    while timer > 0{
    thread::sleep(Duration::from_secs(1));
    // let mut new_val = timer - 1;
    timer = timer - 1;
    let minutes = timer/60;
    let seconds = timer % 60;
    println!("{}:{}",minutes, format!("{:02}", seconds));
    }
}