use std::{fs, process};

pub fn has_local_ips(path: &str, ip_addr: &str) -> bool {
    // We get array of Strings from read function, so we iter trhough it and process each row
    // when processing row we need to check if it contains specified ip address, and if the ip
    // address is uncommented. If it's uncommented that means we have it. So we return true
    for row in read(path) {
        // If contains ip address and it isn't commented out
        if row.contains(ip_addr) && !row.contains("#") {
            return true;
        }
    }

    // Return false if didn't find uncommented ip address
    // The return value is last line without ";" character
    false
}

pub fn read(path: &str) -> Vec<String> {
    // In this code, we need to handle Ok(), and Err(), unlike in write
    // That's why I am using match to check for errors
    match fs::read_to_string(path) {
        // If we get lines we get an array of &str
        // So we map through it, and convert it to mutable String, and then collect iterable values
        // with .collect()
        Ok(contents) => contents.lines().map(|line| line.to_string()).collect(),
        Err(e) => {
            // Print out error if happened, and exit with 1 code
            eprintln!("Error while reading {} file: {}", path, e);
            process::exit(1);
        }
    }
}

pub fn write(path: &str, contents: Vec<String>) {
    // We could use match Ok(_), Err(e) statement
    // But since we need to check only for error, we will use this method
    if let Err(e) = fs::write(path, contents.join("\n")) {
        eprintln!("Error while writing {} file: {}", path, e);
        process::exit(1);
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
}
