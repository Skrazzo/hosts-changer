// use std::time::Duration;
// use tokio::time;
// use tokio_wifiscanner::scan;
// use std::fs::OpenOptions;

// Import modules
mod file;

// variables
const HOSTS_PATH: &str = "hosts";
const IP_ADDRESS: &str = "192.168.8.109";

// #[tokio::main]
fn main() {
    if file::has_local_ips(HOSTS_PATH, IP_ADDRESS) {
        println!("Has ip addresses")
    } else {
        println!("Does not have ip addresses")
    }
}

// async fn waitForNetworkChange() {
//     // Define the target SSID
//     let target_ssid = "Leons iPhone";
//
//     // Start the monitoring loop
//     loop {
//         match scan().await {
//             Ok(networks) => {
//                 // Check if the target SSID is in the list of available networks
//                 let connected = networks.iter().any(|network| network.ssid == target_ssid);
//
//                 if connected {
//                     // Execute function for the target SSID
//                     on_target_ssid_connected();
//                 } else {
//                     // Execute function for other networks
//                     on_other_ssid_connected();
//                 }
//             }
//             Err(e) => eprintln!("Failed to scan Wi-Fi networks: {}", e),
//         }
//
//         // Wait for a specified interval before scanning again
//         time::sleep(Duration::from_secs(5)).await; // Adjust the interval as needed
//     }
// }
//
// fn on_target_ssid_connected() {
//     // Functionality to execute when connected to the target SSID
//     println!("Connected to the target SSID.");
//     // Add your code here
// }
//
// fn on_other_ssid_connected() {
//     // Functionality to execute when connected to a different SSID
//     println!("Connected to a different SSID.");
//     // Add your code here
// }
