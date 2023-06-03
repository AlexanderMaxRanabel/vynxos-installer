use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::process;
use std::error::Error;
use reqwest::blocking::get;

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

fn main() -> Result<(), Box<dyn Error>> {
    if is_nixos() {
        let mut installer_choice = String::new();
        println!("Welcome to VynxOS Installer!");
        println!("Migrate from 22.11 to 23.05 or New Installition?");
        io::stdin().read_line(&mut installer_choice).expect("Error While reading input");
        let installer_choice = installer_choice.trim();

        match installer_choice {
            "install" => {
                let url = "https://raw.githubusercontent.com/AlexanderMaxRanabel/VynxOS-config/main/configuration.nix";
                let response = get(url)?.text()?;

                let content = response.trim();
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
    Ok(())
}
