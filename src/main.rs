use chrono::Local;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(6000);
    loop {
	println!("UPD: {} | TODO: {} | NEWS: {} | BAT: {} | {}",
		 updates(),
		 tasks(),
		 news(),
		 battery(),
		 datetime(),
	);

        thread::sleep(delay);
    }
}

fn datetime() -> String {
    Local::now().format("%H:%M, %a %d %b %Y").to_string()
}

fn battery() -> u32 {
    let bat0path = "/sys/class/power_supply/BAT0/capacity";
    let bat1path = "/sys/class/power_supply/BAT1/capacity";

    let bat0percent = read_num_from_file(bat0path);
    let bat1percent = read_num_from_file(bat1path);

    bat0percent + bat1percent
}

fn news() -> u32 {
    read_num_from_file("~/.local/share/newsunread")
}

fn tasks() -> u32 {
    read_num_from_file( "~/.local/share/tasks")
}

fn updates() -> u32 {
    read_num_from_file( "~/.local/share/updates")
}

fn read_num_from_file(filepath: &'static str) -> u32 {
    let file = match File::open(&filepath) {
	Ok(file) => file,
	Err(_) => return 0
    };

    let mut buffer = BufReader::new(file);

    let mut line = String::new();
    let _ = buffer.read_line(&mut line);

    line.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap()
}
