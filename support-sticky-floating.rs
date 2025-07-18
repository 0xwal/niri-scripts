#!/usr/bin/env scriptisto

// #region meta
// scriptisto-begin
// script_src: src/main.rs
// build_cmd: cargo build --release --color=always && strip ./target/release/floating
// target_bin: ./target/release/floating
// files:
//  - path: Cargo.toml
//    content: |
//     package = { name = "floating", version = "0.1.0", edition = "2024"}
//     [dependencies]
//     serde = { version = "1.0.219", features = [ "derive" ] }
//     serde_json = "1.0.140"
// scriptisto-end
// #endregion

#![allow(dead_code)]

use std::collections::HashMap;
use std::io::{
	BufRead,
	BufReader,
};
use std::process::{
	Command,
	Stdio,
};

static mut FLOATING_WINDOWS: *mut HashMap<u64, FloatingWindow> = std::ptr::null_mut();

#[derive(Debug, Eq, Hash, PartialEq)]
struct FloatingWindow {
	id: u64,
	workspace_id: u64,
	output: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct WindowOpenedOrChanged {
	id: u64,
	workspace_id: u64,
	is_floating: bool,
}

#[derive(Debug, serde::Deserialize)]
struct WorkspaceFocused {
	id: u64,
}

#[derive(Debug, serde::Deserialize)]
struct Workspace {
	id: u64,
	name: Option<String>,
	output: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
enum Event {
	WindowOpenedOrChanged { window: WindowOpenedOrChanged },
	WorkspaceActivated(WorkspaceFocused),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	unsafe {
		if FLOATING_WINDOWS.is_null() {
			FLOATING_WINDOWS = Box::into_raw(Box::new(HashMap::new()));
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
				match payload {
					Ok(Event::WindowOpenedOrChanged {
						window,
					}) => {
						on_window_changes(window);
					}
					Ok(Event::WorkspaceActivated(workspace)) => {
						on_workspace_activated(workspace);
					}
					_ => (),
				}
			}
			Err(e) => {
				eprintln!("Error reading output: {}", e);
			}
		}
	}

	let status = child.wait()?;

	Ok(())
}

fn move_window_to_workspace(workspace_id: &str, id: u64) {
	Command::new("niri")
		.args(["msg", "action", "move-window-to-workspace", "--focus", "true"])
		.arg("--window-id")
		.arg(id.to_string())
		.arg(workspace_id)
		.spawn()
		.unwrap();
}

fn get_workspace_info(id: u64) -> Workspace {
	let output = Command::new("niri")
		.args(["msg", "--json", "workspaces"])
		.stdout(Stdio::piped())
		.spawn()
		.unwrap()
		.wait_with_output()
		.unwrap();

	let output = String::from_utf8_lossy(&output.stdout);

	let workspaces: Vec<Workspace> = serde_json::from_str(&output).unwrap();

	let workspace = workspaces.into_iter().find(|it| it.id == id).unwrap();

	workspace
}

fn on_workspace_activated(workspace: WorkspaceFocused) {
	let workspace = get_workspace_info(workspace.id);
	let target = workspace.name.unwrap_or(workspace.id.to_string());

	for (window, window_info) in unsafe { &(*FLOATING_WINDOWS) } {
		if workspace.output.as_ref().unwrap() != window_info.output.as_ref().unwrap() {
			continue;
		}
		move_window_to_workspace(target.as_str(), *window);
	}
}

fn on_window_changes(window: WindowOpenedOrChanged) {
	let already_tracked = unsafe { (*FLOATING_WINDOWS).contains_key(&window.id) };

	if !window.is_floating && !already_tracked {
		return;
	}

	if !window.is_floating {
		unsafe {
			(*FLOATING_WINDOWS).remove(&window.id);
		}
		return;
	}

	let parent_workspace = get_workspace_info(window.workspace_id);

	let window = FloatingWindow {
		id: window.id,
		workspace_id: window.workspace_id,
		output: parent_workspace.output,
	};

	unsafe {
		(*FLOATING_WINDOWS).insert(window.id, window);
	}
}
