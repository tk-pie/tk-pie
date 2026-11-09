//! This module abstracts between different HAL implementations.
//!
//! It exports a number of type aliases. For example, [`Input`], where the concrete type is from
//! `esp-hal` or `embassy-rp` (depending on compiler target), both of which implements
//! [`embedded_hal::digital::InputPin`].

/// Trait for abstracting between different HAL crates.
pub trait IHal {
    type AnyPin<'a>;
    type Input<'a>: embedded_hal::digital::InputPin
        + embedded_hal_async::digital::Wait
        + embedded_hal::digital::ErrorType;
}

// Re-export Hal associated types for convenience

/// An unconfigured GPIO pin. See [`IHal::AnyPin`];
pub type AnyPin<'a> = <Hal as IHal>::AnyPin<'a>;
/// A GPIO input pin. See [`IHal::Input`];
pub type Input<'a> = <Hal as IHal>::Input<'a>;

cfg_select! {
    target_arch = "arm" => {
        use rp::Rp2040Hal as Hal;
        mod rp {
            pub struct Rp2040Hal;
            impl super::IHal for Rp2040Hal {
                type AnyPin<'a> = embassy_rp::Peri<'a, embassy_rp::gpio::AnyPin>;
                type Input<'a> = embassy_rp::gpio::Input<'a>;
            }
        }
    }
    target_arch = "riscv32" => {
        use esp::EspHal as Hal;
        mod esp {
            pub struct EspHal;
            impl super::IHal for EspHal {
                type AnyPin<'a> = esp_hal::gpio::AnyPin<'a>;
                type Input<'a> = esp_hal::gpio::Input<'a>;
            }
        }
    }
}
