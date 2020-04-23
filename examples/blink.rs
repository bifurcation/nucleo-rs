#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_halt;
extern crate stm32l4xx_hal;

use crate::cortex_m_rt::entry;
use crate::stm32l4xx_hal::delay::Delay;
use crate::stm32l4xx_hal::prelude::*;

use crate::cortex_m_semihosting::hio;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    const BLINK_MS: u32 = 500;

    let mut hstdout = hio::hstdout().unwrap();
    writeln!(hstdout, "Hello, world!").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4xx_hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.hclk(8.mhz()).freeze(&mut flash.acr);
    let mut timer = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob
        .pb3
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut i: usize = 0;
    loop {
        timer.delay_ms(BLINK_MS);
        writeln!(hstdout, "{} Blink on!", i).unwrap();
        led.set_high().unwrap();

        timer.delay_ms(BLINK_MS);
        writeln!(hstdout, "{} Blink off!", i).unwrap();
        led.set_low().unwrap();

        i += 1;
    }
}
