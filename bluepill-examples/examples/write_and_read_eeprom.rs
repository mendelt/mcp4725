//! This example shows writing the dac and eeprom and then using the read method to read back the
//! written values.
//! It calls read right after the set_dac_and_eeprom command to show that the eeprom_write_status is
//! still false when writing. After the eeprom has been written the status turns to true.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use hal::i2c::{BlockingI2c, Mode};
use hal::pac;
use hal::prelude::*;
use hal::time::U32Ext;

use mcp4725::*;
use panic_semihosting as _;

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

    hprintln!("old status {:x?}", dac.read().unwrap()).ok();

    // Set the output
    dac.set_dac_and_eeprom(PowerDown::Resistor100kOhm, 0x0112).ok();

    // This probably prints eeprom still writing
    hprintln!("new status {:x?}", dac.read().unwrap()).ok();

    // This prints eeprom done writing
    hprintln!("new new status {:x?}", dac.read().unwrap()).ok();

    loop {}
}
