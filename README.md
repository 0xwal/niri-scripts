# Niri WM Sticky Floating Support Extension

## Overview

This script makes floating windows **sticky** across all workspaces in the [Niri Window Manager](https://github.com/YaLTeR/niri). Any floating window will be moved to the focused workspace, allowing it to follow you across different workspaces.



https://github.com/user-attachments/assets/fab73541-c683-49b3-80da-ae1d3a110348



## Why?

To improve my workflow by keeping floating windows available on every workspace until
these features supported nativly.

## Features

* **Sticky Floating**: Moves any floating window to the focused workspace.

**Note**: In the future, floating and sticky functionality will be split into separate features.

## Dependencies

* **Scriptisto**: Required to run the script. [Github](https://github.com/igor-petruk/scriptisto)
## Installation

1. Clone the repo:

   ```bash
   git clone https://github.com/0xwal/niri-scripts.git
   ```

2. Make the script executable:

   ```bash
   chmod +x niri-scripts/support-sticky-floating.rs
   ```

3. Run the script:

   ```bash
   niri-scripts/support-sticky-floating.rs &
   disown
   ```

   OR within config

   ```kdl
   spawn-at-startup "dir/niri-scripts/support-sticky-floating.rs"

   ```

