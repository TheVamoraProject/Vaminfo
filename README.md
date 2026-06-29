<p align="center">
  <img src="https://github.com/user-attachments/assets/878fb5a9-8055-4613-bbe1-ada5730eff48" alt="Vaminfo Logo" width="300">
</p>
<p align="center">
A simple customizable system fetch tool made just for <b>VamoraOS</b> 💙
</p>

<p align="center">
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/license-MIT-blue.svg"></a>
  <a href="https://github.com/TheVamoraProject/Vaminfo/releases"><img src="https://img.shields.io/github/v/release/TheVamoraProject/Vaminfo?color=green&label=latest"></a>
  <a href="https://github.com/TheVamoraProject/Vaminfo/issues"><img src="https://img.shields.io/github/issues/TheVamoraProject/Vaminfo"></a>
  <a href="https://github.com/TheVamoraProject/Vaminfo/stargazers"><img src="https://img.shields.io/github/stars/TheVamoraProject/Vaminfo?style=social"></a> 
</p>


---
## Features

- ⚡ Instant startup
- 🧠 Auto-layout: split (desktop) or stacked (mobile) based depending on terminal width
- 🎨 Fully colorized, customizable output
- 🧩 Modular trait-based architecture — each info module is independent and easy to add
- 🎭 ASCII art system with custom files + built-in Vamora logo
- 📱 Mini mode for quick essential info
- 🧙 Interactive config wizard — no manual file editing needed

## Usage

```sh
vaminfo               # Display system information
vaminfo config        # Launch interactive configuration wizard
vaminfo --mini        # Minimal view: OS, CPU, RAM, Uptime
vaminfo --debug       # Debug output + system info
vaminfo --version     # Print version
vaminfo --help        # Show help
```

## Installation / Update
u can install the binary and its files from releases or
### from website 
```sh
curl -fsSL https://vamora.vercel.app/install/vaminfo.sh | sudo bash
```
### from repo file
```sh
chmod +x install.sh
./install.sh
```
The installer will:
1. Check / install the Rust toolchain
2. Build a release binary
3. Install to `/usr/local/bin/vaminfo`
4. Create `~/.VamoraSys/apps/vaminfo/` directory structure
5. Deploy bundled ASCII art
6. Generate a default `config.vmf` if one doesn't exist

## Manual Build

```sh
cargo build --release
./target/release/vaminfo
```


## ASCII Art

Select/add/remove them via `vaminfo config`.

If the selected file is missing or empty, vaminfo silently falls back to the built-in ASCII art.

```ascii-art



          ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒       
       ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    
     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  
    ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒ 
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒     ▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒      ▒      ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒▒      ▒       ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒▒      ▒        ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒▒      ▒         ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒▒      ▒          ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒      ▒           ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒      ▒░           ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ░      ▒      ▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒           ▒      ▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒          ▒      ▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒         ▒      ▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒        ▒      ▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒       ▒      ▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒      ▒      ░▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
   ▒▒▒▒▒▒▒▒▒▒     ▒▒      ▒▒▒▒▒▒▒     ▒▒▒▒▒▒▒▒▒▒
    ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒   ▒▒▒▒▒▒▒▒▒▒ 
     ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒  
       ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    
         ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒       



```
## Layout Algorithm

At runtime, vaminfo reads the terminal dimensions using `crossterm`:

- If `cols >= ascii_width + 50` **and** `rows >= 10` → **Split layout** (ASCII left, info right)
- Otherwise → **Stacked layout** (ASCII on top, info below)

No manual override — fully automatic and adaptive.

## Modules

| Module   | Description                            |
|----------|----------------------------------------|
| hostname | System hostname                        |
| os       | OS name, version, architecture         |
| kernel   | Kernel version                         |
| cpu      | CPU model, cores, frequency, load %    |
| gpu      | GPU model (Linux /sys/class/drm)       |
| ram      | Used / Total memory with percentage    |
| disk     | Disk usage for `/` and `/home`         |
| uptime   | System uptime (days/hours/minutes)     |
| shell    | Current shell from `$SHELL`            |
| desktop  | DE/WM and display server (X11/Wayland) |
| battery  | Battery % and charge status            |
| network  | Interface names with RX/TX totals      |

All modules can be toggled via `vaminfo config` — no source code changes needed.


<!-- made by vamora -->
---
<p align="center">
  <sub>
    Made by 
    <a href="https://rb.gy/7jh0i9" target="_blank">
      <img src="https://github.com/user-attachments/assets/efb3ad9b-6b07-4488-9c16-79586297ee5d" alt="Vamora" height="10">
    </a>
  </sub>
</p>
