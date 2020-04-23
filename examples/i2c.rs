#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use crate::hal::prelude::*;

use crate::hal::i2c::I2c;
use crate::rt::entry;

use crate::sh::hio;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .hsi48(true)
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    // SCL = D1 = PA9
    let mut scl = gpioa
        .pa9
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, false);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    // SDA = D0 = PA10
    let mut sda = gpioa
        .pa10
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, false);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);

    let mut buffer: [u8; 6] = [0x80, 0x00, 0x02, 0x31, 0x41, 0x0b];
    for addr in 0x00..=0xff {
        match i2c.write(addr as u8, &mut buffer) {
            Ok(_) => writeln!(hstdout, "[{:02x}] FOUND", addr).unwrap(),
            Err(err) => writeln!(hstdout, "[{:02x}] {:?}", addr, err).unwrap(),
        }
    }

    loop {}
}
