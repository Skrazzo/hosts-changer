# Host changer

This project is made because I self host a lot of the stuff, and local dns server is not an option for me. So I use hosts file to map local ip addresses of my server to a domain. 
This app will keep eye on your connected wifi name, and change hosts file if you're connected to specified wifi.

## What it does
- When you're on target wifi, it will read your hosts file, and uncomment lines where you have local ip address specified
- When you're not on target wifi, it will comment out lines with local ip address.

## How to set up

Just set your variables here in main.rs file, compile, and enjoy
```rust
// variables
const HOSTS_PATH: &str = "Path to your hosts file";
const IP_ADDRESS: &str = "Your local ip";
const TARGET_SSID: &str = "Wifi address";
```

If you're wondering about comments in the code, I wrote them so that I can learn and understand rust language better.

_100% Hand written code, no AI used_
