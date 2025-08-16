fn main() {
    let time = Instant::now();
    prevent_windows();

    let user = get_user();
    let os = get_os();
    println!(
        "User: {user}! \nOS: {} \nCPU architecture: {}",
        match os {
            OperatingSystem::Linux => "Linux",
            OperatingSystem::MacOS => "macOS",
        },
        match get_architecture() {
            Architecture::Bit64 => "64-bit",
            Architecture::Bit32 => "32-bit",
            Architecture::Aarch64 => "Arch64",
        }
    );
    let distro = get_distro();
    println!("Distro: {distro}");
    println!("Current time: {}", get_time());
    // dbg!(time.elapsed());
}

use std::{env, thread::sleep, time::Instant};
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

enum Architecture {
    Bit64,
    Bit32,
    Aarch64,
}
fn get_architecture() -> Architecture {
    #[cfg(target_arch = "x86_64")]
    {
        Architecture::Bit64
    }
    #[cfg(target_arch = "x86")]
    {
        Architecture::Bit32
    }
    #[cfg(target_arch = "aarch64")]
    {
        Architecture::Aarch64
    }
}

fn get_distro() -> String {
    #[cfg(target_os = "linux")]
    {
        use std::fs;

        let content = fs::read_to_string("/etc/os-release").unwrap_or_default();
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line["PRETTY_NAME=".len()..].trim_matches('"').to_string();
            }
        }
        "Linux Unknown".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        "macOS".to_string()
    }
}
