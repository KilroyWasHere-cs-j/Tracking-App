use std::thread;
use std::time::Duration;

pub fn summon_daemon(){
    println!("Summoning daemon...");
    let mut daemon = thread::spawn(|| {
        loop {
            println!("Daemon is running...");
            thread::sleep(Duration::from_secs(1));
        }
    });
}