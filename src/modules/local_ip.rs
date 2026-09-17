use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::net::UdpSocket;
use std::process::Command;

pub struct LocalIpModule;

impl Module for LocalIpModule {
    fn name(&self) -> &'static str { "Local IP" }

    fn collect(&self, _sys: &System, _cfg: &VaminfoConfig) -> Option<String> {
        // Use UDP trick: connect to external addr (no traffic sent) to find local IP
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    let ip = addr.ip().to_string();
                    if ip != "0.0.0.0" {
                        return Some(ip);
                    }
                }
            }
        }
        // Offline fallback: hostname -I reads local interface state and does
        // not open a connection or wait for the network.
        if let Ok(out) = Command::new("hostname").arg("-I").output() {
            if out.status.success() {
                if let Some(ip) = String::from_utf8_lossy(&out.stdout)
                    .split_whitespace()
                    .find(|ip| *ip != "127.0.0.1" && !ip.starts_with("169.254."))
                {
                    return Some(ip.to_string());
                }
            }
        }
        None
    }
}
