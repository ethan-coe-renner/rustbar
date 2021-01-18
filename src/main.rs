use std::{thread, time};

fn main() {
    let delay = time::Duration::from_millis(6000);

    let header = rustbar::Header {
	version: 1,
    };
    
    println!("{}\n[\n[]\n", serde_json::to_string(&header).unwrap());
    
    loop {
	let line = vec!(
	    rustbar::music(),
	    rustbar::updates(),
	    rustbar::tasks(),
	    rustbar::news(),
	    rustbar::audio(),
	    rustbar::battery(),
	    rustbar::datetime(),
	);

	println!(",{}", serde_json::to_string(&line).unwrap());

        thread::sleep(delay);
    }
}
