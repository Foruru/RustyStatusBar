# Rusty status bar
Simple status bar for dwm written in rust. Easy to use.

## What it can show
 - Time
 - Date
 - CPU temperature
 - Keyboard layout

## Usage

### Dependencies
 - rust
 - cargo
 - libxcb-dev
 - libxkbfile-dev

### Installation
```
cd rusty-statusbar
cargo install --path .
```
and then put it into your dwm start script
```
rusty-statusbar --loop &
```

### Command-line options
```
Usage:
  rusty-statusbar [OPTIONS]


Optional arguments:
  -h,--help             Show this help message and exit
  -l,--loop             Loop program.
  -r,--refresh-rate REFRESH_RATE
                        Set refresh rate in milliseconds (default: 1000).
```
