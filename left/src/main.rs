//! Firmware for Tk🥧, left half.

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
            half: Half::Left,
            pins: [
                // row 1
                board.d24.into(),
                board.a3.into(),
                board.a2.into(),
                board.a1.into(),
                board.a0.into(),
                // row 2
                board.d25.into(),
                board.sck.into(),
                board.mosi.into(),
                board.miso.into(),
                board.d2.into(),
                // row 3
                board.d12.into(),
                board.d11.into(),
                board.d10.into(),
                board.d9.into(),
                board.d3.into(),
                // thumbpad
                board.d7.into(),
                board.scl.into(),
                board.sda.into(),
            ],
            // the index of the LEDs is different than the switch index.
            // each number is the index of the LED for the switch of that index.
            led_map: [4, 3, 2, 1, 0, 5, 6, 7, 8, 9, 14, 13, 12, 11, 10, 15, 16, 17],
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
