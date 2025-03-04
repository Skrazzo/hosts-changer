use std::fs::read_to_string;

pub fn has_local_ips(path: &str, ip_addr: &str) -> bool {
    let txt: String = read_to_string(path).unwrap();

    for row in txt.lines() {
        // If contains ip address and it isn't commented out
        if row.contains(ip_addr) && !row.contains("#") {
            return true;
        }
    }

    false
}

pub fn read_file(path: &str) {}
