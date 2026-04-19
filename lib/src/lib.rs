#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
    all(target_os = "none", target_arch = "arm"),
    feature(impl_trait_in_assoc_type)
)]

extern crate alloc;
extern crate cortex_m_rt;

#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod allocator;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod board;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod interrupts;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod keyboard;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod panic_handler;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub mod ws2812;

pub mod atomics;
pub mod button;
pub mod button_layout;
pub mod event;
pub mod hemicom;
pub mod keypress_handler;
pub mod keys;
pub mod layer;
pub mod lights;
pub mod logger;
pub mod rgb;
pub mod rtt;
pub mod serial_proto;
pub mod usb;
pub mod util;

#[cfg(all(target_os = "none", target_arch = "arm"))]
mod entry;
#[cfg(all(target_os = "none", target_arch = "arm"))]
pub use entry::*;
