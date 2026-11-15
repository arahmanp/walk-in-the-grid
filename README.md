# Rust Learning Project: Walk in the Grid

## Introduction

This is one of my Rust learning projects that i've done. I built this stuff mainly to strengthen my Rust skill (cuz i'm new in Rust). This project is actually super simple, just a single red square on a green `32x16` character grid. You can control this red square using `WASD` key, just like when you're playing games.

## Technical Details

Here's some technical details in this project:
- **Rust version:** `v1.99.1`.
- **Dependencies:**
  - `crossterm v0.29`

## Installation

Make sure you have `cargo` installed on your system, you'll need it to build this project. If you already have `cargo` installed on your system, follow these instructions below to build this project:

```bash
# 1. Clone this repository
git clone https://github.com/arahmanp/walk-in-the-grid.git

# 2. Move to project directory
cd walk-in-the-grid

# 3. Build the project
cargo build --release

# 4. Run it!
# for Linux/macOS, run:
./target/release/walk-in-the-grid

# for Windows, run:
.\target\release\walk-in-the-grid.exe

```