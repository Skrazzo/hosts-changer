use std::fs;

pub fn has_local_ips(path: &str, ip_addr: &str) -> bool {
    for row in read(path) {
        // If contains ip address and it isn't commented out
        if row.contains(ip_addr) && !row.contains("#") {
            return true;
        }
    }

    false
}

pub fn read(path: &str) -> Vec<String> {
    let txt: String = fs::read_to_string(path).unwrap();
    // Convert unmutable barrowed &str to actuall mutable strings that arent barrowed
    txt.lines().map(|s| s.to_string()).collect()
}

pub fn write(path: &str, contents: Vec<String>) {
    if let Err(e) = fs::write(path, contents.join("\n")) {
        eprint!("Failed to write to the file {}", e);
    }
}

pub fn comment_ips(path: &str, ip_addr: &str) {
    // Read hosts file. And receive array of String (that aren't barrowed)
    let mut hosts = read(path);

    // We use &mut so that we can spawn a mutable line variable
    for line in &mut hosts {
        if line.contains(ip_addr) {
            // * operator is needed to access real mutable variable
            // * -> line -> hosts[i] then it changes it directly
            // without it, hosts mutable variable wouldn't be modified
            *line = format!("# {}", line);
        }
    }

    // Write hosts file with new data
    write(path, hosts);
    println!("Added comments on ips");
}

pub fn uncomment_ips(path: &str, ip_addr: &str) {
    // Read hosts file. And receive array of String (that aren't barrowed)
    let mut hosts = read(path);

    // We use &mut so that we can spawn a mutable line variable
    for line in &mut hosts {
        if line.contains(ip_addr) && line.contains("#") {
            // * operator is needed to access real mutable variable
            // * -> line -> hosts[i] then it changes it directly
            // without it, hosts mutable variable wouldn't be modified
            *line = line.replace("#", "").trim().to_string();
        }
    }

    // Write hosts file with new data
    write(path, hosts);
    println!("Removed comments from ip addresses");
}
