fn main() {
    let _time = Instant::now();
    prevent_windows();

    let user = get_user();
    let hostname = get_hostname();
    let os = get_os();
    println!(
        "User: {user}! \nOS: {} \nCPU architecture: {} \nHostname: {hostname} \nCPU Model: {}",
        match os {
            OperatingSystem::Linux => "Linux",
            OperatingSystem::MacOS => "macOS",
        },
        match get_architecture() {
            Architecture::Bit64 => "64-bit",
            Architecture::Bit32 => "32-bit",
            Architecture::Aarch64 => "Arch64",
        },
        get_modal()
    );
    let distro = get_distro();
    println!("Distro: {distro}");
    if is_macos() {
        let macos_ver = get_macos_ver();
        println!("macOS Version: {macos_ver}");
    }
    println!("Current time: {}", get_time());
    // dbg!(time.elapsed());
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

fn get_macos_ver() -> String {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("sw_vers")
            .arg("-productVersion")
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute sw_vers command"));

        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            version.trim().to_string()
        } else {
            "Unknown macOS Version".to_string()
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        "Not macOS".to_string()
    }
}

fn is_macos() -> bool {
    #[cfg(target_os = "macos")]
    {
        true
    }
    #[cfg(not(target_os = "macos"))]
    {
        false
    }
}

fn get_hostname() -> String {
    #[cfg(target_os = "linux")]
    {
        use std::fs;

        fs::read_to_string("/etc/hostname")
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("scutil")
            .arg("--get")
            .arg("ComputerName")
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute scutil command"));

        if output.status.success() {
            let hostname = String::from_utf8_lossy(&output.stdout);
            hostname.trim().to_string()
        } else {
            "unknown".to_string()
        }
    }
}

fn get_modal() -> String {
    #[cfg(target_os = "linux")]
    {
        use std::fs;

        let content = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
        for line in content.lines() {
            if line.starts_with("model name") {
                return line["model name".len() + 1..].trim().to_string();
            }
        }
        "Unknown CPU Model".to_string()
    }
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute sysctl command"));

        if output.status.success() {
            let model = String::from_utf8_lossy(&output.stdout);
            model.trim().to_string()
        } else {
            "Unknown CPU Model".to_string()
        }
    }
}
