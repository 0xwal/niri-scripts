# Niri WM Sticky Floating Support Extension

## Overview

This script makes floating windows **sticky** across all workspaces in the [Niri Window Manager](https://github.com/YaLTeR/niri). Any floating window will be moved to the focused workspace, allowing it to follow you across different workspaces.

https://github.com/user-attachments/assets/fab73541-c683-49b3-80da-ae1d3a110348

## Why?

To improve workflow by keeping floating windows available on every workspace until these features are supported natively.

## Features

* **Sticky Floating**: Moves any floating window to the focused workspace.
* **Wallpaper Per Workspace**: Assign a wallpaper for each individual workspace.
* **Screenshot**: Take screenshot then annotate it.

**Note**: In the future, floating and sticky functionality will be split into separate features.

## Dependencies

* **Scriptisto**: Required to run the script. [GitHub](https://github.com/igor-petruk/scriptisto)
* **Wallpaper Per Workspace**:
    - [swww](https://github.com/LGFae/swww)
* **Screenshot**:
    - [grim](https://github.com/GrimAnticheat/Grim)
    - [satty](https://github.com/gabm/Satty)
    - [slurp](https://github.com/emersion/slurp)


## Installation

1. Clone the repo:

   `git clone https://github.com/0xwal/niri-scripts.git`

2. Make the scripts executable:

   `chmod +x niri-scripts/support-sticky-floating.rs`

   `chmod +x niri-scripts/wallpaper-per-workspace.rs`

   `chmod +x niri-scripts/screenshot.rs`

## Running

### Run the scripts:

`niri-scripts/support-sticky-floating.rs &`

`niri-scripts/wallpaper-per-workspace.rs <WALLPAPER_DIR> &`

`niri-scripts/screenshot.rs <PATH_TO_SAVE_SCREENSHOT> &`

`disown`

OR within your config:

`spawn-at-startup "dir/niri-scripts/support-sticky-floating.rs"`

`spawn-at-startup "dir/niri-scripts/wallpaper-per-workspace.rs <WALLPAPERS_DIR>"`

`spawn-at-startup "dir/niri-scripts/screenshot.rs <PATH_TO_SAVE_SCREENSHOT>"`

### ARGS

* `WALLPAPER_DIR`: This directory contains each wallpaper named by the workspace index. For example:
  * `WALLPAPER_DIR/1` will be active when workspace 1 is focused.
  * `WALLPAPER_DIR/2` will be active when workspace 2 is focused, and so on.
  * `WALLPAPER_DIR/FALLBACK` will be used when activating a workspace with no wallpaper file.

- `PATH_TO_SAVE_SCREENSHOT`: The path of the directory to save your screenshots.
