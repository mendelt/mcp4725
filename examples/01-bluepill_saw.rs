#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::pac;
use stm32f1xx_hal::i2c::{Mode, BlockingI2c};
use stm32f1xx_hal::delay::Delay;

#[allow(unused_imports)]
use panic_semihosting;
use mcp4725::*;

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

    // let mut delay = Delay::new(cp.SYST, clocks);

    // Configure I2C

    let mut i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Standard{frequency: 10000},
        clocks,
        &mut rcc.apb1,
        1000,10,1000,1000,
    );

    let mut dac = MCP4725::create(i2c);
    let mut dac_cmd = Command::default();

    dac_cmd = dac_cmd.address(0b00000010);
    let mut value: u16 = 0;

    loop {
        dac_cmd = dac_cmd.data(value);
        dac.send(&dac_cmd);
        value += 1;
        value &= 0x0fff;
    }
}
