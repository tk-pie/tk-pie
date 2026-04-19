// Guard against trying to compile this on embedded
#![cfg_attr(target_os = "none", no_std)]
#[cfg(not(target_os = "none"))]
mod app;

#[cfg(not(target_os = "none"))]
fn main() -> eyre::Result<()> {
    app::run()
}

// Stub main+panic_handler on embedded platforms.
// This makes rustc/rust-analyzer less upset when checking the entire workspace.
#[cfg(target_os = "none")]
fn main() {}
#[cfg(target_os = "none")]
#[panic_handler]
fn panic_blink(_info: &core::panic::PanicInfo) -> ! {
    unreachable!()
}
