# Host changer

This project is made because I self host a lot of the stuff, and local dns server is not an option for me. So I use hosts file to map local ip addresses of my server to a domain.
This app will keep eye on your connected wifi name, and change hosts file if you're connected to specified wifi.

## What it does

- When you're on target wifi, it will read your hosts file, and uncomment lines where you have local ip address specified
- When you're not on target wifi, it will comment out lines with local ip address.

## How to set up

First, you must have [Rust](https://www.rust-lang.org/tools/install) installed.

Set your variables here in main.rs file

```rust
// variables
const HOSTS_PATH: &str = "Path to your hosts file";
const IP_ADDRESS: &str = "Your local ip";
const TARGET_SSID: &str = "Wifi address";
const SLEEP_TIME: u64 = 5; // How often to check for wifi connection
```

Compile the code with Cargo and run the executable

```bash
cargo build --release

# executable will be in this path
./target/release/hosts-checker
```

## Running in the background in MacOS

Copy the `com.skrazzo.hostschecker.plist` file to your `~/Library/LaunchAgents` folder, and within the file, change the executable path to your own path. Doing this will show the executable in `Settings > General > Login Items & Extensions > Allow in the Background`, and be run in the background and reset in case of a crash.

```xml
<key>Program</key>
<!-- Change this to your executable path -->
<string>/Users/Skrazzo/hosts</string>
```

If you're wondering about comments in the code, I wrote them so that I can learn and understand rust language better.

_100% Hand written code, no AI used_
