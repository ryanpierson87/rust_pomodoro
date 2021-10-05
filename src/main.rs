mod countdown_clock;

use std::io;
use gtk::gdk;

fn main() {
    let minutes = 1;
    let mut confirm = String::new();
    println!("Press ENTER to begin next phase:");
    io::stdin()
        .read_line(&mut confirm);
    countdown_clock::countdown(minutes, "work");
    println!("Press ENTER to begin next phase:");
    io::stdin()
        .read_line(&mut confirm);
    countdown_clock::countdown(minutes, "rest");


    // // count_down(1);
    // let mut timer = 60;
    // while timer > 0{
    // thread::sleep(Duration::from_secs(1));
    // // let mut new_val = timer - 1;
    // timer = timer - 1;
    // let minutes = timer/60;
    // let seconds = timer % 60;
    // println!("{}:{}",minutes, seconds);
    
}