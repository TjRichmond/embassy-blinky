#![no_std]
#![no_main]

// Import boiler plate crates
use cortex_m_rt::entry;
use panic_halt as _;
use stm32_metapac::{self as device, gpio};

// Import custom files
mod delay;

#[entry]
fn main() -> ! {
    // Create local variables for each peripheral base address
    let rcc = device::RCC;
    let gpiob = device::GPIOB;
    let gpiof = device::GPIOF;
    let gpiog = device::GPIOG;

    // Initialize gpio port clocks
    rcc.ahb2enr().modify(|w| w.set_gpioben(true));
    rcc.ahb2enr().modify(|w| w.set_gpiofen(true));
    rcc.ahb2enr().modify(|w| w.set_gpiogen(true));

    // Initialize gpio pin modes to output
    gpiob.moder().modify(|w| w.set_moder(0, gpio::vals::Moder::OUTPUT));
    gpiof.moder().modify(|w| w.set_moder(4, gpio::vals::Moder::OUTPUT));
    gpiog.moder().modify(|w| w.set_moder(4, gpio::vals::Moder::OUTPUT));

    // Forever loop
    loop {
        // Wait for 100ms
        delay::busy_wait(100_000);

        // Set pins to 1
        gpiob.bsrr().write(|w| w.set_bs(0, true));
        gpiof.bsrr().write(|w| w.set_bs(4, true));
        gpiog.bsrr().write(|w| w.set_bs(4, true));

        // Wait for 100ms
        delay::busy_wait(100_000);

        // Reset pins to 0
        gpiob.bsrr().write(|w| w.set_br(0, true));
        gpiof.bsrr().write(|w| w.set_br(4, true));
        gpiog.bsrr().write(|w| w.set_br(4, true));
    }
}
