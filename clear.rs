use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    // Read the initial amount of used RAM
    let initial_memory_usage = get_memory_usage();
    println!("Initial Memory Usage: {:.2} MB", initial_memory_usage);

    // Open the cache clearing file
    let mut f = match OpenOptions::new().write(true).open("/proc/sys/vm/drop_caches") {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening the file: {}", err);
            return;
        }
    };

    // Write the value 3 to the file to clear the memory cache
    if let Err(err) = f.write_all(b"3\n") {
        eprintln!("Error writing to the file: {}", err);
        return;
    }

    // Read the updated amount of used RAM
    let updated_memory_usage = get_memory_usage();
    println!("Updated Memory Usage: {:.2} MB", updated_memory_usage);

    // Calculate the amount of memory released
    let memory_released = initial_memory_usage - updated_memory_usage;
    println!("Memory Released: {:.2} MB", memory_released);
}

fn get_memory_usage() -> f64 {
    // Read the contents of the memory usage file
    let contents = match std::fs::read_to_string("/proc/meminfo") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading memory info: {}", err);
            return 0.0;
        }
    };

    // Parse the total memory and free memory values
    let mut total_memory = 0.0;
    let mut free_memory = 0.0;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }

        match parts[0] {
            "MemTotal:" => {
                total_memory = parts[1].parse().unwrap_or(0.0);
            }
            "MemFree:" => {
                free_memory = parts[1].parse().unwrap_or(0.0);
            }
            _ => {}
        }
    }

    // Calculate the used memory by subtracting free memory from total memory
    let used_memory = total_memory - free_memory;

    // Convert memory to MB
    used_memory / 1024.0
}
