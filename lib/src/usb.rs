use embassy_sync::pubsub::PubSubChannel;
use serde::{Deserialize, Serialize};

use crate::util::CS;

#[cfg(target_arch = "arm")]
pub mod driver;
#[cfg(target_arch = "arm")]
pub mod keyboard;
#[cfg(target_arch = "arm")]
pub mod serial;

#[cfg(target_arch = "riscv32")]
pub mod serial {
    /// STUB IMPL. TODO: FIX
    // Fix by implementing the above `serial` module for risc-v
    pub fn serial_send(message: &crate::serial_proto::borrowed::DeviceMsg<'_>) {
        let _ = message;
    }
}

pub const MAX_PACKET_SIZE: u8 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UsbEvent {
    Enabled(bool),
    Suspended(bool),
    Configured(bool),
    Addressed(u8),
    Reset,
}

pub type UsbEventChannel = PubSubChannel<CS, UsbEvent, 8, 24, 0>;
pub static USB_EVENTS: UsbEventChannel = UsbEventChannel::new();
