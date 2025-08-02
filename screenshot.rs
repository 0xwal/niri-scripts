#!/usr/bin/env scriptisto
// vim: shiftwidth=2 softtabstop=2

// scriptisto-begin
// script_src: src/main.rs
// build_cmd: >
//    cargo clippy --color=always &&
//    cargo build --release --color=always &&
//    strip ./target/release/screenshot
// target_bin: ./target/release/screenshot
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "screenshot", version = "0.1.0", edition = "2024"}
//     [dependencies]
// scriptisto-end

#![allow(dead_code)]
#![deny(warnings)]
#![deny(clippy::unwrap_used)]

use std::process::{
	Command,
	Stdio,
};

use std::error::Error;

use std::time::SystemTime;

fn init_screenshot(filename: &str) -> Result<bool, Box<dyn Error>> {
	let coords = {
		let output = Command::new("slurp")
			.args(["-c", "#FA7265AF", "-s", "#00000030", "-b", "#00000030"])
			.stdout(Stdio::piped())
			.spawn()?
			.wait_with_output()?;

		String::from_utf8_lossy(&output.stdout).to_string()
	};

	if coords.is_empty() {
		return Ok(false);
	}

	Command::new("grim")
		.arg("-g")
		.arg(coords.to_string().trim())
		.arg(filename)
		.spawn()?
		.wait()
		.map(drop)?;

	Ok(true)
}

fn annotate_screenshot(path: &str) -> Result<(), Box<dyn Error>> {
	Ok(
		Command::new("satty")
			.arg("-f")
			.arg(path)
			.arg("-o")
			.arg(path)
			.arg("--fullscreen")
			.arg("--disable-notifications")
			.spawn()?
			.wait()
			.map(drop)?,
	)
}

fn main() {
	let screenshots_dir = std::env::args().nth(1).expect("No screenshot directory specified");

	let time = SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.expect("Failed to get time");

	let path = format!("{}/{}.png", screenshots_dir, time.as_secs());

	let done = init_screenshot(&path).expect("Failed to initialize screenshot");
	if !done {
		return;
	}

	annotate_screenshot(&path).expect("Failed to annotate screenshot");
}
