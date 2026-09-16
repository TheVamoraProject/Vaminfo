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
- 🧠 Auto-layout: split (desktop) or stacked (mobile) depending on terminal width
- 🎨 Fully colorized, customizable output
- Nerd Font module icons with a four-point Vamora fallback icon
- Nerd Font icons can be enabled or disabled from `vaminfo config` (main menu or Display Options)
- 🧩 Modular trait-based architecture — each info module is independent and easy to add
- 🎭 ASCII art system with custom files + built-in Vamora logo
- 📱 Mini mode for quick essential info
- 🧾 JSON export for scripting/integration
- 💡 Random Linux tips
- 🧙 Interactive config wizard — no manual file editing needed

## Usage

```sh
vaminfo               # Display system information
vaminfo config        # Launch interactive configuration wizard
vaminfo --mini        # Mini mode: OS, Host, RAM, Uptime
vaminfo --json        # Export all hardware stats as JSON
vaminfo --tip         # Display a random Linux tip
vaminfo --debug       # Debug output + system information
vaminfo --version, -v # Print version information
vaminfo --help, -h    # Show help message
```
## Wizard Controls
- Arrow keys (or `j`/`k`) to move the Nerd Font selector
- `Enter` to select / confirm
- `Space` or `Enter` to toggle a module (the readable `[ON ]` / `[OFF]` state works with or without Nerd Fonts)
- `U`/`D` or `Shift+↑`/`Shift+↓` to reorder modules
- `q` or `Esc` to exit without saving

## Installation / Update
You can install the binary and its files from releases or:
### from website 
```sh
curl -fsSL https://vamora.vercel.app/install/vaminfo.sh | bash
```
### from repo file
```sh
chmod +x install.sh
./install.sh
```
The installer will:
1. Check / install the Rust toolchain
2. Download JetBrainsMono Nerd Font v3.5.1 from GitHub into `~/.local/share/fonts`
3. Build a release binary
4. Install to `/usr/local/bin/vaminfo`
5. Create `~/.VamoraSys/apps/vaminfo/` directory structure
6. Deploy bundled ASCII art
7. Generate a default `config.vmf` if one doesn't exist
8. Enable Nerd Font icons only when the font setup succeeds

After installation, open your terminal emulator's **Preferences** or
**Profile** settings, enable custom fonts (or disable its system/default font
option), and set **JetBrainsMono Nerd Font** as the default font. Restart the
terminal if existing tabs still show missing glyphs. Use
`VAMINFO_SKIP_FONT=1 ./install.sh` to skip the automatic font download.

If the font could not be installed, the generated config keeps **Nerd Font
icons** off so the output remains readable. You can change it later with
`vaminfo config` → **Display Options** → **Nerd Font icons**.

## Manual Build

```sh
cargo build --release
./target/release/vaminfo
```

## ASCII Art

Select/add/remove them via `vaminfo config`.

If the selected file is missing or empty, vaminfo silently falls back to the built-in ASCII art.



## Layout Algorithm

At runtime, vaminfo reads the terminal dimensions using `crossterm`:

- If `cols >= ascii_width + 50` **and** `rows >= 10` → **Split layout** (ASCII left, info right)
- Otherwise → **Stacked layout** (ASCII on top, info below)

No manual override — fully automatic and adaptive.

## Modules

| Module          | Description                                          |
|-----------------|-------------------------------------------------------|
|-----------------| **System Identity** |                                  
| hostname        | System hostname                                       |
| os              | OS name, version, architecture                         |
| kernel          | Kernel version                                         |
| bios            | BIOS information                                       |
|-----------------|**Hardware**    |
| cpu             | CPU model, cores, frequency, load %                    |
| gpu             | GPU model (Linux /sys/class/drm)                       |
| ram             | Used / Total memory with percentage                    |
| disk            | Disk usage for `/` and `/home`                          |
| battery         | Battery % and charge status                             |
| bluetooth       | Bluetooth status                                        |
|-----------------|**Time**        |
| uptime          | System uptime (days/hours/minutes)                     |
| system age      | Time since OS install                                   |
|-----------------|**Environment** |
| shell           | Current shell from `$SHELL`                             |
| terminal        | Current terminal emulator                               |
| tty type        | TTY type                                                |
| desktop         | DE/WM and display server (X11/Wayland)                  |
| display server  | Display server in use                                    |
| resolution      | Screen resolution                                        |
| theme           | Active system theme                                       |
| filesystem type | Root filesystem type                                       |
| sudo privileges | Whether the current user has sudo access                   |
|-----------------|**Network**     |
| local ip        | Local IP address                                         |
| public ip       | Public IP address (makes a network request)               |
| network I/O     | Interface names with RX/TX totals                         |
|-----------------|**Android**   *(Android/Termux only)*     |
| android version | Android OS version                                        |
| android device  | Android device model                                       |
|-----------------|**VamoraOS**    |
| vamoraos info   | VamoraOS-specific info                                    |
|-----------------|**Fun / Optional**                                |
| birthday cake | Displays an ASCII art when u run vaminfo the day of ur birthday   |
| birthday cntdwn | Countdown to birthday (requires birthday set in greetings)  |
| custom events   | User-defined date-triggered messages                        |
| quotes          | Random quote display                                       |
| linux jokes     | Random Linux joke                                           |
|-----------------|**Color Blocks**|
| color blocks big| Large color block display                                  |
| color blocks sml| Small color block display                                  |

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
