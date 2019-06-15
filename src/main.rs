//! Quickstart template for the Nordic nRF52 series

#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_semihosting;

use {
    core::fmt::Write,
    cortex_m_semihosting::hprintln,
    // nrf52832 and nrf52840 also available
    nrf52810_hal::{
        self as hal,
        gpio::{GpioExt, Level},
        uarte::{Baudrate, Parity, UarteExt},
    },
    rtfm::app,
};

#[app(device = crate::hal::target)]
const APP: () = {
    #[init]
    fn init() {
        hprintln!("init").unwrap();

        let p0 = device.P0.split();

        let mut serial = {
            let rxd = p0.p0_08.into_floating_input().degrade();
            let txd = p0.p0_06.into_push_pull_output(Level::Low).degrade();

            let pins = hal::uarte::Pins {
                rxd,
                txd,
                cts: None,
                rts: None,
            };

            device
                .UARTE0
                .constrain(pins, Parity::EXCLUDED, Baudrate::BAUD1M)
        };
        writeln!(serial, "\ninit finished").unwrap();
    }

    #[idle]
    fn idle() -> ! {
        hprintln!("idle").unwrap();

        loop {}
    }
};
