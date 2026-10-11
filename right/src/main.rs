//! Firmware for Tk🥧, right half.

#![cfg_attr(target_os = "none", no_main)]
#![cfg_attr(target_os = "none", no_std)]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")] // only try to compile this for embedded
#[cortex_m_rt::entry]
fn main() -> ! {
    use alloc::vec::Vec;
    use tk_pie::{
        board::MappedBoard, event::Half, interrupts::Irqs, keyboard::KeyboardConfig, layer::Layer,
        ws2812::Ws2812,
    };

    tk_pie::run(|board| {
        let layers = include_bytes!("layers.pc");
        let Ok(layers): Result<Vec<Vec<Layer>>, _> = postcard::from_bytes(layers) else {
            log::error!("Failed to deserialize layer config");
            #[expect(clippy::empty_loop)]
            loop {}
        };

        let keyboard = KeyboardConfig {
            half: Half::Right,
            pins: [
                // row 1
                board.d12.into(),
                board.d11.into(),
                board.d10.into(),
                board.d9.into(),
                board.d7.into(),
                // row 2
                board.a2.into(),
                board.a1.into(),
                board.a0.into(),
                board.sda.into(),
                board.scl.into(),
                // row 3
                board.mosi.into(),
                board.sck.into(),
                board.d25.into(),
                board.d24.into(),
                board.a3.into(),
                // thumbpad
                board.d3.into(),
                board.d2.into(),
                board.miso.into(),
            ],
            led_map: [0, 1, 2, 3, 4, 9, 8, 7, 6, 5, 10, 11, 12, 13, 14, 15, 16, 17],
            led_driver: Ws2812::new(board.PIO1, Irqs, board.DMA_CH1, board.d5),
            layers,
        };

        let neopixel = Ws2812::new(board.PIO0, Irqs, board.DMA_CH0, board.neopixel);

        MappedBoard {
            USB: board.USB,
            UART0: board.UART0,
            UART1: board.UART1,

            keyboard,

            rx: board.rx,
            tx: board.tx,

            neopixel,
            neopixel_power: board.neopixel_power,
        }
    })
}

#[cfg(not(target_os = "none"))] // stub main for non-embedded targets
fn main() {
    panic!("Wrong! You must specify another --target when building this crate.")
}
