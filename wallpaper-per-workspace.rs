#!/usr/bin/env scriptisto

// #region meta
// scriptisto-begin
// script_src: src/main.rs
// build_cmd: >
//  cargo clippy --color=always &&
//  cargo build --release --color=always &&
//  strip ./target/release/wallpaper-per-workspace
// target_bin: ./target/release/wallpaper-per-workspace
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "wallpaper-per-workspace", version = "0.1.0", edition = "2024"}
//     [dependencies]
//     serde = { version = "1.0.219", features = [ "derive" ] }
//     serde_json = "1.0.140"
// scriptisto-end
// #endregion

#![allow(dead_code)]
#![deny(warnings)]
#![deny(clippy::unwrap_used)]

use std::io::{
	self,
	BufRead,
	BufReader,
};

use std::process::{
	Command,
	Stdio,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

static mut ACTIVE_WALLPAPER: Option<String> = None;
static mut WALLPAPERS_DIR: *mut String = std::ptr::null_mut();

// #[rustfmt::skip]
// const WALLPAPERS: &[(&str, &str)] = &[
//   ("1", "601-2.png"),
//   ("2", "594-2.jpg"),
//   ("3", "595.png"),
//   ("4", "591-darken.png"),
//   ("5", "572-1.png"),
//   ("6", "571.png"),
//   ("7", "571.png"),
//
//   ("w21", "601-2.png"),
//   ("w22", "594-2.jpg"),
//   ("w23", "595.png"),
//   ("w24", "591-darken.png"),
//   ("w25", "572-1.png"),
//   ("w26", "571.png"),
// ];

#[derive(Debug, serde::Deserialize)]
struct WorkspaceFocused {
	id: u64,
	focused: bool,
}

#[derive(Debug, serde::Deserialize)]
struct Workspace {
	id: u64,
	idx: u64,
	name: Option<String>,
	is_active: bool,
	is_focused: bool,
	active_window_id: Option<u64>,
	output: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
enum Event {
	WorkspaceActivated(WorkspaceFocused),
}

fn check_file_exist(path: &str) -> bool {
	std::fs::metadata(path).is_ok()
}

fn get_workspace_info(id: u64) -> Option<Workspace> {
	let child = Command::new("niri")
		.arg("msg")
		.arg("--json")
		.arg("workspaces")
		.stdout(Stdio::piped())
		.spawn();

	let Ok(output) = child.ok()?.wait_with_output() else {
		return None;
	};
	let stdout = String::from_utf8_lossy(&output.stdout);

	let workspaces: Vec<Workspace> = serde_json::from_str(stdout.as_ref()).ok()?;

	workspaces.into_iter().find(|w| w.id == id)
}

fn change_wallpaper(wallpaper: &str, output: &str) -> Result<()> {
	let wallpaper = format!("{}/{wallpaper}", unsafe { &*WALLPAPERS_DIR });

	if !check_file_exist(&wallpaper) {
		return Err("Wallpaper not found".into());
	}

	Command::new("swww")
		.arg("img")
		.arg(wallpaper)
		.arg("-o")
		.arg(output)
		.arg("--transition-type")
		.arg("fade")
		.arg("--transition-duration")
		.arg("0.4")
		.stdout(Stdio::piped())
		.spawn()?
		.wait_with_output()?;

	Ok(())
}

fn on_workspace_focused(payload: WorkspaceFocused) {
	let Some(workspace) = get_workspace_info(payload.id) else {
		return;
	};

	let ws_id = workspace.idx.to_string();
	let Some(output) = workspace.output else {
		return;
	};

	unsafe {
		let a = &raw const ACTIVE_WALLPAPER;
		if let Some(w) = &*a {
			if *w == ws_id {
				return;
			}
		}
	}

	unsafe {
		ACTIVE_WALLPAPER = Some(ws_id.to_string());
	}

	change_wallpaper(&ws_id, &output)
		.or_else(|_| change_wallpaper("FALLBACK", &output))
		.ok();
}

fn main() -> io::Result<()> {
	let wallpapers_dir = std::env::args()
		.nth(1)
		.or_else(|| std::env::var("WALLPAPERS_DIRS").ok())
		.unwrap_or_else(|| env!("HOME").to_string() + "/.wallpapers");

	unsafe {
		if WALLPAPERS_DIR.is_null() {
			WALLPAPERS_DIR = Box::into_raw(Box::new(wallpapers_dir));
		}
	}

	let mut child = Command::new("niri")
		.arg("msg")
		.arg("--json")
		.arg("event-stream")
		.stdout(Stdio::piped())
		.spawn()?;

	let stdout = child.stdout.take().expect("Failed to capture stdout");

	let reader = BufReader::new(stdout);

	for line in reader.lines() {
		match line {
			Ok(line_content) => {
				let payload = serde_json::from_str::<Event>(&line_content);
				if let Ok(Event::WorkspaceActivated(payload)) = payload {
					on_workspace_focused(payload);
				}
			}
			Err(e) => {
				eprintln!("Error reading output: {e}");
			}
		}
	}

	let status = child.wait()?;
	println!("Process exited with status: {status}");

	Ok(())
}
