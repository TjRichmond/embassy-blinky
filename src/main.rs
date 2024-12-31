#![no_std]
#![no_main]

// Import boiler plate crates
use cortex_m_rt::entry;
use panic_halt as _;
use stm32_metapac::{self as device, gpio, interrupt, pwr, rcc};

#[entry]
fn main() -> ! {
    // Create local variables for each peripheral base address
    let rcc = device::RCC;
    let pwr = device::PWR;
    let flash = device::FLASH;
    let gpiob = device::GPIOB;
    let tim2 = device::TIM2;

    // Enable the HSE (High-Speed External Clock)
    rcc.cr().modify(|w| w.set_hseon(true));
    while rcc.cr().read().hserdy() == false {} // Wait until HSE is ready
    
    // Configure PLL1 to the following:
    // HSE_CLK = 8 MHz
    // PLLM = /4 -> 2 MHz
    // PLLN = *250 -> 500 MHz
    // PLLP = /2 -> 250 MHz
    rcc.plldivr(0).modify(|w| {
        w.set_plln(rcc::vals::Plln::MUL250);
        w.set_pllp(rcc::vals::Plldiv::DIV2);
    });
    rcc.pllcfgr(0).modify(|w| {
        w.set_pllsrc(rcc::vals::Pllsrc::HSE);
        w.set_divm(rcc::vals::Pllm::DIV4);
        w.set_pllpen(true);
    });
    
    // Enable PLL1
    rcc.cr().modify(|w| w.set_pllon(0, true));
    
    // Configure number of flash wait states to 5
    flash.acr().modify(|w| {
        w.set_latency(5);
        w.set_wrhighfreq(3);
    });

    // Increase voltage level to use max clock frequency
    pwr.voscr().modify(|w| w.set_vos(pwr::vals::Vos::SCALE0));
    while pwr.vossr().read().actvosrdy() == false {} // Wait until VOS is ready

    // Select PLL as the system clock source
    rcc.cfgr().modify(|w| w.set_sw(rcc::vals::Sw::PLL1_P));
    while rcc.cfgr().read().sws() != rcc::vals::Sw::PLL1_P {} // Wait until PLL is used as the system clock

    // Initialize GPIOB clock
    rcc.ahb2enr().modify(|w|w.set_gpioben(true));

    // Initialize TIM2 clock
    rcc.apb1lenr().modify(|w| w.set_tim2en(true));

    // Initialize GPIOB to output
    gpiob.moder().modify(|w| w.set_moder(0, gpio::vals::Moder::OUTPUT));

    // Configure TIM2
    // PreScalar = (System clock / Counter Frequency) - 1
    // PSC = 250MHz / 10KHz - 1
    // Auto Reload Register = (Counter Frequency / Desired Interrupt Frequency) - 1
    // ARR = 10KHz / 1 Hz - 1
    tim2.psc().write(|w| w.set_psc(25_000 - 1)); // Set prescaler
    tim2.arr().write(|w| w.set_arr(10_000 - 1)); // Set auto reload value
    tim2.dier().write(|w| w.set_uie(true)); // Enable tim2 interrupt
    tim2.cr1().write(|w| w.set_cen(true)); // Enable tim2

    // Enable TIM2 interrupt in NVIC
    unsafe {
        let nvic = &*cortex_m::peripheral::NVIC::PTR;

        // Calculate position in NVIC table for TIM2 interrupt
        let interrupt_position = stm32_metapac::interrupt::TIM2 as u32;
        let register_index = (interrupt_position / 32) as usize;
        let bit_position = interrupt_position % 32;

        // Enable TIM2 interrupt
        nvic.iser[register_index].write(1 << bit_position); 
    }

    // Forever loop
    loop {
        // Wait for interrupt
        cortex_m::asm::wfi();
    }
}

#[interrupt]
fn TIM2() {
    // Get addresses to peripheral registers
    let tim2 = &stm32_metapac::TIM2;
    let gpiob = &stm32_metapac::GPIOB;

    // Clear the update interrupt flag
    tim2.sr().modify(|w| w.set_uif(false));

    // Toggle PB0 (GREEN LED)
    gpiob.odr().modify(|w| w.set_odr(0, 
        match gpiob.odr().read().odr(0) {
            gpio::vals::Odr::HIGH => gpio::vals::Odr::LOW,
            gpio::vals::Odr::LOW => gpio::vals::Odr::HIGH,
        }));
}