use super::Module;
use crate::config::VaminfoConfig;
use sysinfo::System;
use std::net::{TcpStream, UdpSocket};

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
        // Fallback: try TCP
        if let Ok(stream) = TcpStream::connect("8.8.8.8:80") {
            if let Ok(addr) = stream.local_addr() {
                let ip = addr.ip().to_string();
                if ip != "0.0.0.0" {
                    return Some(ip);
                }
            }
        }
        None
    }
}
