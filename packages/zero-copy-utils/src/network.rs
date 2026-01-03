//! # Network Abstraction Layer
//!
//! Abstracts over different network drivers (standard kernel sockets vs userspace drivers)
//! to support both development (Mac/Linux standard) and production HFT (Solarflare/DPDK).

#![allow(unexpected_cfgs)]

use async_trait::async_trait;
use bytes::BytesMut;
use std::io;

/// Interface for low-latency packet I/O
#[async_trait]
pub trait PacketDriver: Send + Sync {
    /// Initialize the driver interface (e.g., open NIC queue)
    async fn init(&mut self) -> io::Result<()>;

    /// Send a raw packet
    async fn send(&self, buf: &[u8]) -> io::Result<()>;

    /// Receive a raw packet into the provided buffer
    /// Returns the number of bytes read
    async fn recv(&self, buf: &mut BytesMut) -> io::Result<usize>;
}

/// Standard Linux/Unix Socket Driver (Mio/Tokio based)
pub struct StandardDriver {
    #[allow(dead_code)]
    interface: String,
}

impl StandardDriver {
    pub fn new(interface: &str) -> Self {
        Self {
            interface: interface.to_string(),
        }
    }
}

#[async_trait]
impl PacketDriver for StandardDriver {
    async fn init(&mut self) -> io::Result<()> {
        // In reality, bind generic socket
        Ok(())
    }

    async fn send(&self, _buf: &[u8]) -> io::Result<()> {
        Ok(())
    }

    async fn recv(&self, _buf: &mut BytesMut) -> io::Result<usize> {
        // Wait for packet
        Ok(0)
    }
}

/// Solarflare OpenOnload / EF_VI Driver
///
/// Use implementation when `#[cfg(feature = "solarflare")]` is enabled.
/// This bypasses the kernel network stack completely.
#[cfg(feature = "solarflare")]
pub struct SolarflareDriver {
    interface: String,
    // ef_vi handles would go here
}

#[cfg(feature = "solarflare")]
impl SolarflareDriver {
    pub fn new(interface: &str) -> Self {
        Self {
            interface: interface.to_string(),
        }
    }
}

#[cfg(feature = "solarflare")]
#[async_trait]
impl PacketDriver for SolarflareDriver {
    async fn init(&mut self) -> io::Result<()> {
        println!("Initializing Solarflare EF_VI on {}", self.interface);
        // FFI calls to libonload/libef_vi would go here
        Ok(())
    }

    async fn send(&self, _buf: &[u8]) -> io::Result<()> {
        // ef_vi_transmit(...)
        Ok(())
    }

    async fn recv(&self, _buf: &mut BytesMut) -> io::Result<usize> {
        // ef_vi_poll(...)
        Ok(0)
    }
}
