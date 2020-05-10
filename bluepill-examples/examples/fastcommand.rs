//! Example that creates a square wave (alternating high and low) using the MCP4725 driver sending
//! fast-commands. This example is written and tested on the STM32f103 on the bluepill board.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::i2c::{BlockingI2c, Mode};
use hal::pac;
use hal::prelude::*;
use hal::time::U32Ext;

use mcp4725::*;
#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Configure the pins for I2C1
    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    // Configure I2C
    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Standard {
            frequency: 400000.hz(),
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    // Configure the MCP4725 DAC
    let mut dac = MCP4725::new(i2c, 0b010);

    loop {
        dac.set_dac_fast(PowerDown::Normal, 0x0fff);
        dac.set_dac_fast(PowerDown::Normal, 0x0000);
    }
}
