use chrono::Local;

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::{thread, time};
use std::process::Command;

use serde::Serialize;

#[derive(Serialize)]
struct Elem {
    full_text: String,
    color: String,
}


#[derive(Serialize)]
struct Header {
    version: u32,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Block {
    Line(Vec<Elem>),
}

fn main() {
    let delay = time::Duration::from_millis(6000);

    let header = Header {
	version: 1,
    };
    
    println!("{}\n[\n[]\n", serde_json::to_string(&header).unwrap());
    
    loop {

	let line = vec!(
	    updates(),
	    tasks(),
	    news(),
	    audio(),
	    battery(),
	    datetime(),
	);

	let line = Block::Line(line);

	println!(",{}", serde_json::to_string(&line).unwrap());

        thread::sleep(delay);
    }
}

fn datetime() -> Elem {
    gen_elem("", &Local::now().format("%H:%M, %a %d %b %Y").to_string(), "#5e81ac")
}

fn battery() -> Elem {
    let bat0percent = read_num_from_file("/sys/class/power_supply/BAT0/capacity");
    let bat1percent = read_num_from_file("/sys/class/power_supply/BAT1/capacity");


    gen_elem("BAT: ", &(bat0percent + bat1percent).to_string(), "#d8dee9")
}

fn news() -> Elem {
    gen_elem("NEWS: ", &read_num_from_file("~/.local/share/newsunread").to_string(), "#d8dee9")
}

fn tasks() -> Elem {
    gen_elem("TODO: ", &read_num_from_file("~/.local/share/tasks").to_string(), "d8dee9")
}

fn updates() -> Elem {
    gen_elem("UPD: ", &read_num_from_file("~/.local/share/updates").to_string(), "d8dee9")
}

fn audio() -> Elem {
    let volumeoutput = Command::new("pamixer")
        .arg("--get-volume")
	.output()
	.expect("failed to execute process");

    let muteoutput = Command::new("pamixer")
        .arg("--get-mute")
	.output()
	.expect("failed to execute process");

    let volume = String::from_utf8_lossy(&volumeoutput.stdout).chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap();

    let muted: String = String::from_utf8(muteoutput.stdout).unwrap();

    match muted.as_ref() {
	"false\n" => gen_elem("VOL: ", &volume.to_string(), "#d8dee9"),
	"true\n" => gen_elem("VOL: ", &volume.to_string(), "#4c566a"),
	_ => gen_elem("VOL: ", &volume.to_string(), "#4c566a"),
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
