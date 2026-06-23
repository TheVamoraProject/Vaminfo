# Changelog

All notable changes to Vaminfo will be documented here.

---

## [v5.0.0] - 2025

### Highlights
Complete rewrite in Rust — zero external dependencies, faster startup, safer code.

### Added
- Full Rust rewrite of the entire codebase
- Custom ASCII art support (`--ascii-add`, `--ascii-list`, `--ascii`, `--ascii-delete`, `--ascii-reset`)
- Battery info now reads directly from `/sys/class/power_supply` (no upower needed)
- Terminal detection via `/proc` instead of process tree walking
- Unknown command now shows help message automatically
- Architecture detection in installer (x86_64, aarch64, armv7)

### Changed
- Config moved from `/etc/VamoraSys/` to `~/.vaminfo/config.vmf`
- Custom ASCII arts stored in `~/.vaminfo/ascii/`
- Version check now fetches from GitHub raw (`info.vmf`)

### Removed
- Bash dependency
- `upower` dependency for battery info

---

## [v4.0] - 2025

### Added
- `--color` / `-c` flag to set and save ASCII art color
- `--colors` flag to list all available colors
- 19 color options including Orange (256-color)
- Color config saved to `/etc/VamoraSys/vaminfo/VaminfoConfig.vmf`
- Dynamic color fallback to LightBlue if config unreadable

### Changed
- Improved terminal detection with more supported terminals
- CPU truncated to 42 chars, GPU to 30 chars

---

## [v3.0] - 2024

### Added
- Network status and public IP display
- `--mini` / `-m` compact mode
- Color swatches row at the bottom

### Changed
- Side-by-side layout with terminal width detection
- Fallback to stacked layout on narrow terminals

---

## [v2.0] - 2024

### Added
- `--update` / `-u` flag
- `--version` / `-v` flag
- Battery, resolution, and terminal detection
- GPU detection via `lspci`

---

## [v1.0] - 2024

### Added
- Initial release
- Basic system info: OS, kernel, uptime, shell, WM, CPU, RAM, disk
- Vamora ASCII logo
- Side-by-side layout
