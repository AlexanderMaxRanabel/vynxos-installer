use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::process;

fn is_nixos() -> bool {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("ID=nixos") {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    if is_nixos() {
        let mut installer_choice = String::new();
        println!("Welcome to VynxOS Installer!");
        println!("Migrate from 22.11 to 23.05 or New Installition?");
        io::stdin().read_line(&mut installer_choice).expect("Error While reading input");
        let installer_choice = installer_choice.trim();

        match installer_choice {
            "install" => {

            },

            "migrate" => {
                println!("Please be sure you runned VynxOS Installer with sudo otherwise it wont simply work");

                let channel_update = Command::new("nix-channel")
                    .arg("--add")
                    .arg("https://nixos.org/channels/nixos-23.05")
                    .arg("nixos")
                    .output()
                    .expect("Failed");
                if channel_update.status.success() {
                    let small_update = Command::new("nix-channel")
                        .arg("--update")
                        .output()
                        .expect("Failed");
                    if small_update.status.success() {
                        let build = Command::new("nixos-rebuild")
                            .arg("switch")
                            .output()
                            .expect("Failed");

                        if build.status.success() {
                            let reboot = Command::new("reboot")
                                .output()
                                .expect("Failed");
                            if reboot.status.success() {

                            } else {
                                let stderr = String::from_utf8_lossy(&channel_update.stderr);
                                eprintln!("Failed to run command. Error: {}", stderr);
                                process::exit(1);
                            }
                        } else {
                            let stderr = String::from_utf8_lossy(&channel_update.stderr);
                            eprintln!("Failed to run command. Error: {}", stderr);
                            process::exit(1);
                        }
                    } else {
                       let stderr = String::from_utf8_lossy(&channel_update.stderr);
                        eprintln!("Failed to run command. Error: {}", stderr);
                        process::exit(1);
                    }
                } else {
                    let stderr = String::from_utf8_lossy(&channel_update.stderr);
                    eprintln!("Failed to run command. Error: {}", stderr);
                    process::exit(1);
                }

            },
            _ => {
                println!("Not a valid choice");
                process::exit(1);
            }
        }
    } else {
        println!("Error: Distro is not NixOS");
        process::exit(1);
    }

}
