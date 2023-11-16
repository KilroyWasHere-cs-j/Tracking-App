use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use crate::database::db::Database;

pub fn summon_daemon(db: Mutex<Database>){
    println!("Summoning daemon...");
    thread::spawn(|| {
        loop {
            println!("Daemon is running...");
            thread::sleep(Duration::from_secs(1));
        }
    });
}