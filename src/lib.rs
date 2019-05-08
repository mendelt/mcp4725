#![no_std]

//use nb::Result;
use core::cmp;
use embedded_hal::blocking::i2c::Write;

pub struct MCP4725<I2C>
where
    I2C: Write,
{
    i2c: I2C,
}

impl<I2C> MCP4725<I2C>
where
    I2C: Write,
{
    pub fn create(i2c: I2C) -> Self {
        MCP4725 { i2c }
    }

    pub fn send(&mut self, command: &Command) {
        self.i2c
            .write(command.address_byte(), &command.data_bytes());
    }
}

const DEVICE_ID: u8 = 0b1100;

pub struct Command {
    address: u8,
    write_eeprom: bool,
    power_mode: PowerMode,
    data: u16,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            address: 0,
            write_eeprom: false,
            power_mode: PowerMode::Normal,
            data: 0,
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum PowerMode {
    Normal = 0b00,
    Resistor1kOhm = 0b01,
    Resistor100kOhm = 0b10,
    Resistor500kOhm = 0b11,
}

impl Command {
    pub fn address_byte(&self) -> u8 {
        self.address
    }

    /// Format data bytes to send to the DAC. At the moment only sending one sample at a time is
    /// supported.
    pub fn data_bytes(&self) -> [u8; 3] {
        [
            if self.write_eeprom == true {
                0x60
            } else {
                0x20
            } + (self.power_mode as u8)
                << 1,
            (self.data >> 4) as u8,
            (self.data & 0x000f << 4) as u8,
        ]
    }

    /// Set the data to send with this command. This data will be truncated to a 12 bit int
    pub fn data(mut self, data: u16) -> Self {
        self.data = cmp::min(data, 0x0fff);
        self
    }

    /// Set the 3 bit address to use with this command
    pub fn address(mut self, address: u8) -> Self {
        self.address = (DEVICE_ID << 3) + (address & 0b00000111);
        self
    }

    /// Write the supplied values to the EEPROM as well as to the DAC
    pub fn write_eeprom(mut self, write: bool) -> Self {
        self.write_eeprom = write;
        self
    }

    /// Set the power mode
    pub fn power_mode(mut self, mode: PowerMode) -> Self {
        self.power_mode = mode;
        self
    }
}
