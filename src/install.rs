use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io;
use std::io::prelude::*;
use std::process;

pub fn install() {
            println!("Warning: Please be sure you are using NixOS 23.05 during installion of VynxOS");

            println!("Please Choose a hostname: ");
            let mut hostname = String::new();
            io::stdin().read_line(&mut hostname).expect("Error failed to read string");
            let hostname = hostname.trim();

            println!("Please Choose an username: ");
            let mut username = String::new();
            io::stdin().read_line(&mut username).expect("Error failed to read string");
            let username = username.trim();

            println!("Please enter your timezone in tz database format. e.g America/New_York");
            let mut timezone = String::new();
            io::stdin().read_line(&mut timezone).expect("Error failed to read string");
            let timezone = timezone.trim();

            println!("Please enter the keyboard layout code. e.g us or trq");
            let mut keycode = String::new();
            io::stdin().read_line(&mut keycode).expect("Failed to read string");
            let keycode = keycode.trim();

               let mut config = String::from(r#"# Edit this configuration file to define what should be installed on
# your system.  Help is available in the configuration.nix(5) man page
# and in the NixOS manual (accessible by running ‘nixos-help’).

{ config, pkgs, ... }:

{
  imports =
    [ # Include the results of the hardware scan.
      ./hardware-configuration.nix
    ];

  # Bootloader.
  boot.loader.grub.enable = true;
  boot.loader.grub.device = "/dev/sda";
  boot.loader.grub.useOSProber = true;

  networking.hostName = "vynxos"; # Define your hostname.
  # networking.wireless.enable = true;  # Enables wireless support via wpa_supplicant.

  # Configure network proxy if necessary
  # networking.proxy.default = "http://user:password@proxy:port/";
  # networking.proxy.noProxy = "127.0.0.1,localhost,internal.domain";

  # Enable networking
  networking.networkmanager.enable = true;

  # Set your time zone.
  time.timeZone = "Europe/Istanbul";

  # Select internationalisation properties.
  i18n.defaultLocale = "tr_TR.UTF-8";

  i18n.extraLocaleSettings = {
    LC_ADDRESS = "tr_TR.UTF-8";
    LC_IDENTIFICATION = "tr_TR.UTF-8";
    LC_MEASUREMENT = "tr_TR.UTF-8";
    LC_MONETARY = "tr_TR.UTF-8";
    LC_NAME = "tr_TR.UTF-8";
    LC_NUMERIC = "tr_TR.UTF-8";
    LC_PAPER = "tr_TR.UTF-8";
    LC_TELEPHONE = "tr_TR.UTF-8";
    LC_TIME = "tr_TR.UTF-8";
  };

  # Enable the X11 windowing system.
  services.xserver.enable = true;

  # Enable the GNOME Desktop Environment.
  services.xserver.displayManager.gdm.enable = true;
  services.xserver.desktopManager.gnome.enable = true;

  # Configure keymap in X11
  services.xserver = {
    layout = "tr";
    xkbVariant = "intl";
  };

  # Configure console keymap
  console.keyMap = "trq";

  # Enable CUPS to print documents.
  services.printing.enable = true;

  # Enable sound with pipewire.
  sound.enable = true;
  hardware.pulseaudio.enable = false;
  security.rtkit.enable = true;
  services.pipewire = {
    enable = true;
    alsa.enable = true;
    alsa.support32Bit = true;
    pulse.enable = true;
    # If you want to use JACK applications, uncomment this
    #jack.enable = true;

    # use the example session manager (no others are packaged yet so this is enabled by default,
    # no need to redefine it in your config for now)
    #media-session.enable = true;
  };

  # Enable touchpad support (enabled default in most desktopManager).
  # services.xserver.libinput.enable = true;

  # Define a user account. Don't forget to set a password with ‘passwd’.
  users.users.uservynxos = {
    isNormalUser = true;
    description = "vynxos";
    extraGroups = [ "networkmanager" "wheel" ];
    packages = with pkgs; [
      librewolf
    #  thunderbird
    ];
    shell = pkgs.nushell;
  };

  # Enable automatic login for the user.
  services.xserver.displayManager.autoLogin.enable = true;
  services.xserver.displayManager.autoLogin.user = "choroalp";

  # Workaround for GNOME autologin: https://github.com/NixOS/nixpkgs/issues/103746#issuecomment-945091229
  systemd.services."getty@tty1".enable = false;
  systemd.services."autovt@tty1".enable = false;

  # Allow unfree packages
  nixpkgs.config.allowUnfree = true;

  # List packages installed in system profile. To search, run:
  # $ nix search wget
  environment.systemPackages = with pkgs; [
     emacs
     neovim
     vscodium
     nitch
     hyfetch
     pfetch
     helix
     htop
     fish
     gcc
     deno
     clang
     nodejs
     micro
     go
     rustup
     cmake
     docker-client
#  vim # Do not forget to add an editor to edit configuration.nix! The Nano editor is also installed by default.
  #  wget
  ];

  # Some programs need SUID wrappers, can be configured further or are
  # started in user sessions.
  # programs.mtr.enable = true;
  # programs.gnupg.agent = {
  #   enable = true;
  #   enableSSHSupport = true;
  # };

  # List services that you want to enable:

  # Enable the OpenSSH daemon.
  services.openssh.enable = true;

  # Open ports in the firewall.
  # networking.firewall.allowedTCPPorts = [ ... ];
  # networking.firewall.allowedUDPPorts = [ ... ];
  # Or disable the firewall altogether.
  # networking.firewall.enable = false;

  # This value determines the NixOS release from which the default
  # settings for stateful data, like file locations and database versions
  # on your system were taken. It‘s perfectly fine and recommended to leave
  # this value at the release version of the first install of this system.
  # Before changing this value read the documentation for this option
  # (e.g. man configuration.nix or on https://nixos.org/nixos/options.html).
  system.stateVersion = "23.05"; # Did you read the comment?

}"#);
            let user_target = "uservynxos";
            let host_target = "vynxos";
            let location_target = "Europe/Istanbul";
            let keymap_target = "trq";
            let config_path = "/etc/nixos/configuration.nix";
            
            let replaced_config_1 = config.replace(user_target, username);
            let replaced_config_2 = replaced_config_1.replace(host_target, hostname);
            let replaced_config_3 = replaced_config_2.replace(location_target, timezone);
            config = replaced_config_3.replace(keymap_target, keycode);

            let mut file = File::create(config_path).expect("Err");

            file.write_all(config.trim().as_bytes())
                .expect("Failed to write to file.");

            let output = Command::new("nixos-rebuild")
                .arg("boot")
                .output()
                .expect("Failed to rebuild");

            if output.status.success() {
                println!("Nix has been rebuilded. Please reboot your system to make changes effect");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Failed to execute command. error: {}", stderr);
            } 
}
