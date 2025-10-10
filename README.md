# My Niri scripts to extend its functionality

## Overview
https://github.com/user-attachments/assets/fab73541-c683-49b3-80da-ae1d3a110348

## Why?
I like Niri and wanted to extend its functinality for my needs.


## Features

* **Sticky Floating**: Moves any floating window to the focused workspace.
* **Wallpaper Per Workspace**: Assign a wallpaper for each individual workspace.
* **Screenshot**: Take screenshot then annotate it.

## Dependencies

* **Scriptisto**: Required to run the script. [GitHub](https://github.com/igor-petruk/scriptisto)
* **Wallpaper Per Workspace**:
    - [swww](https://github.com/LGFae/swww)
* **Screenshot**:
    - [grim](https://github.com/GrimAnticheat/Grim)
    - [satty](https://github.com/gabm/Satty)
    - [slurp](https://github.com/emersion/slurp)
    - [wl-clipboard](https://github.com/bugaevc/wl-clipboard) (only used when you don't use --annotate)

---

## Installation

1. Clone the repo:

   `git clone https://github.com/0xwal/niri-scripts.git`

2. Make the scripts executable:

   `chmod +x niri-scripts/support-sticky-floating`
   `chmod +x niri-scripts/toggle-sticky`

   `chmod +x niri-scripts/wallpaper-per-workspace`

   `chmod +x niri-scripts/screenshot`

## Running

### Run the scripts:

`niri-scripts/support-sticky-floating &`

`niri-scripts/wallpaper-per-workspace <WALLPAPER_DIR> &`

`disown`

OR within your config:

```kdl
spawn-sh-at-startup "niri-scripts/support-sticky-floating"

binds {
    // To take a screenshot, use whatever keybind you want
    Super+S { spawn-sh "niri-scripts/screeenshot <PATH_TO_SAVE_SCREENSHOT>"; }

    // To take a screenshot then annotate it, use whatever keybind you want
    Super+Ctrl+S { spawn-sh "niri-scripts/screeenshot <PATH_TO_SAVE_SCREENSHOT> --annotate"; }

    // To make the focused window sticky, use whatever keybind you want
    Super+Shift+S { spawn-sh "niri-scripts/toggle-sticky"; }
}

```

### ARGS

* `WALLPAPER_DIR`: This directory contains each wallpaper named by the workspace index. For example:
  * `WALLPAPER_DIR/1` will be active when workspace 1 is focused.
  * `WALLPAPER_DIR/2` will be active when workspace 2 is focused, and so on.
  * `WALLPAPER_DIR/FALLBACK` will be used when activating a workspace with no wallpaper file.

- `PATH_TO_SAVE_SCREENSHOT`: The path of the directory to save your screenshots.
