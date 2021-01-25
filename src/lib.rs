use std::process::Command;

use chrono::Local;

use std::fs::File;
use std::io::{BufReader, BufRead};
use serde::Serialize;

#[derive(Serialize)]
pub struct Elem {
    full_text: String,
    color: String,
}

#[derive(Serialize)]
pub struct Header {
    pub version: u32,
}

const WHITE: &'static str = "#d8dee9";
const GREY: &'static str = "#4c566a";

pub fn datetime() -> Elem {
    gen_elem(
        "",
        &Local::now().format("%H:%M, %a %d %b %Y").to_string(),
        WHITE,
    )
}

pub fn battery() -> Elem {
    let bat0percent = read_num_from_file("/sys/class/power_supply/BAT0/capacity");
    let bat1percent = read_num_from_file("/sys/class/power_supply/BAT1/capacity");
    gen_elem("BAT: ", &(bat0percent + bat1percent).to_string(), WHITE)

}

pub fn news() -> Elem {
    let unread = read_num_from_file("/home/ethan/.local/share/newsunread");
    match unread {
        0 => gen_elem("NEWS: ", &unread.to_string(), GREY),
        _ => gen_elem("NEWS: ", &unread.to_string(), WHITE),
    }
}

pub fn tasks() -> Elem {
    let tasks = read_num_from_file("/home/ethan/.local/share/tasks");
    match tasks {
        0 => gen_elem("TODO: ", &tasks.to_string(), GREY),
        _ => gen_elem("TODO: ", &tasks.to_string(), WHITE),
    }
}

pub fn updates() -> Elem {
    let updates = read_num_from_file("/home/ethan/.local/share/updates");
    match updates {
        0 => gen_elem("UPD: ", &updates.to_string(), "4c566a"),
        _ => gen_elem("UPD: ", &updates.to_string(), "d8dee9"),
    }
}

pub fn audio() -> Elem {
    let volumeoutput = Command::new("pamixer")
        .arg("--get-volume")
        .output()
        .expect("failed to get volume");

    let muteoutput = Command::new("pamixer")
        .arg("--get-mute")
        .output()
        .expect("failed to get mute status");

    let volume = String::from_utf8_lossy(&volumeoutput.stdout)
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u32>()
        .unwrap();

    let muted: String = String::from_utf8(muteoutput.stdout).unwrap();

    match muted.as_ref() {
        "false\n" => gen_elem("VOL: ", &volume.to_string(), WHITE),
        "true\n" => gen_elem("VOL: ", &volume.to_string(), GREY),
        _ => gen_elem("VOL: ", &volume.to_string(), GREY),
    }
}

pub fn music() -> Elem {
    let music_info = Command::new("cmus-remote")
        .arg("-C")
        .arg("format_print '%a - %t'")
        .output()
        .expect("failed to get music_info");

    let music_info = String::from_utf8_lossy(&music_info.stdout).replace("\n","");

    match music_info.as_ref() {
	"" => gen_elem("MUS: ", "none", GREY),
	_ => gen_elem("MUS: ", &music_info, WHITE)
    }
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

fn gen_elem(name: &'static str, text: &str, color: &'static str) -> Elem {
    Elem {
	full_text: name.to_string() + text,
	color: color.to_string()
    }
}
