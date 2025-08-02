fn main() {
    let _ = Instant::now();
    prevent_windows();

    let user = get_user();
    let os = get_os();
    println!(
        "Hello, {user}! \nYou are running this program on a {} system.",
        match os {
            OperatingSystem::Linux => "Linux",
            OperatingSystem::MacOS => "macOS",
        }
    );
    println!("Current time: {}", get_time());
    dbg!(Instant::now().elapsed());
}

use std::{env, time::Instant};
fn get_user() -> String {
    env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}

/// Function to prevent the program from running on Windows.
fn prevent_windows() {
    #[cfg(target_os = "windows")]
    {
        panic!("This program is not supported on Windows.");
    }
}

#[allow(dead_code)]
enum OperatingSystem {
    Linux,
    MacOS,
}

fn get_os() -> OperatingSystem {
    #[cfg(target_os = "linux")]
    {
        OperatingSystem::Linux
    }
    #[cfg(target_os = "macos")]
    {
        OperatingSystem::MacOS
    }
}

fn get_time() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
