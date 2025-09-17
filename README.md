# Vaminfo

> A simple system fetch tool made just for **VamoraOS** 💙

---

## 🧐 What is Vaminfo
<p align="center">
  <img src="https://github.com/user-attachments/assets/30140ed0-aa6e-488c-bcdb-191ec674675c" width="49%" />
  <img src="https://github.com/user-attachments/assets/1c407fa8-ef07-450e-96dc-ee90d22ddfc2" width="49%" />
</p>


`vaminfo` is a lightweight bash script that displays beautiful system info for VamoraOS, made especially for showing off the Vamora brand, environment, and style.

---

## 🎯 Features

- Fancy ASCII art / logo of **Vamora** (or you can easly replace it with ur own one)  
- Key system details:
  - OS / version
  - Kernel
  - Desktop Environment
  - CPU & RAM usage
  - Uptime, etc.
- Fast startup, minimal dependencies (just bash / coreutils)
- Designed specifically for VamoraOS

---

## ⚡ Usage & Flags
- `-h, --help`	  Show help / usage information
- `-v, --version`   Show version
- `-u, --update`   Update if update available

## 🛠 Installation

Here’s how you get it up and running:
- On other distro :
```bash
git clone https://github.com/TheVamoraProject/Vaminfo.git
cd Vaminfo
chmod +x vaminfo
sudo mv vaminfo /usr/bin/
sudo mkdir -p /etc/VamoraSys
sudo mv VaminfoInfo.vmf /etc/VamoraSys/

```
- on VamoraOS :
Vaminfo comes pre-installed 
You can enable “Run on start” directly from your terminal settings.
