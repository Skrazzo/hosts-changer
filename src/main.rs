use std::time::Duration;
use tokio::time;
use tokio_wifiscanner::scan;

// Import modules
mod file;

// variables
const HOSTS_PATH: &str = "hosts";
const IP_ADDRESS: &str = "192.168.8.109";
const TARGET_SSID: &str = "Leons iPhone";

#[tokio::main]
async fn main() {
    // Spawn separate tokio async function, to run both of these functions at the same time
    tokio::spawn(check_for_ctrl_c());

    // Start the monitoring loop
    // Returns Ok if no error, Err if error occurred
    loop {
        match scan().await {
            Ok(networks) => {
                // Check if the target SSID is in the list of available networks
                let connected = networks.iter().any(|network| network.ssid == TARGET_SSID);

                if connected {
                    // Execute function for the target SSID
                    on_target_ssid_connected();
                } else {
                    // Execute function for other networks
                    on_other_ssid_connected();
                }
            }
            Err(e) => eprintln!("Failed to scan Wi-Fi networks: {}", e),
        }

        // Wait for a specified interval before scanning again
        time::sleep(Duration::from_secs(5)).await; // Adjust the interval as needed
        println!("Sleeping");
    }
}

async fn check_for_ctrl_c() {
    // Spawn tokio signal to listen and wait for ctrl + c signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listed for ctrl + c");
    println!("Exiting host checker");
    std::process::exit(0);
}

fn on_target_ssid_connected() {
    // Since we are connected to target wifi ssid, we need to uncomment local ip addresses
    if !file::has_local_ips(HOSTS_PATH, IP_ADDRESS) {
        // Doesnt have local ips (commented)
        file::uncomment_ips(HOSTS_PATH, IP_ADDRESS);
    }
}

fn on_other_ssid_connected() {
    // If we aren't connected to target wifi, we need to comment out local ip addresses
    if file::has_local_ips(HOSTS_PATH, IP_ADDRESS) {
        // That means we have local ip addresses
        file::comment_ips(HOSTS_PATH, IP_ADDRESS);
    }
}
