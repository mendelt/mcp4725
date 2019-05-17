//! *Microchip MCP4725 DAC Driver for Rust Embedded HAL*
//!This is a driver crate for embedded Rust. It's built on top of the Rust
//! [embedded HAL](https://github.com/rust-embedded/embedded-hal)
//! It supports sending commands to a MCP4725 DAC over I2C.
//! To get started you can look at a short
//! [example](https://github.com/mendelt/bluepill-examples/blob/master/examples/01-bluepill_saw.rs)
//! on how to use this driver on an inexpensive blue pill STM32F103 board.
//!
//! The driver can be initialized by calling create and passing it an I2C interface.
//! ```rust, ignore
//! let mut dac = MCP4725::create(i2c);
//! ```
//!
//! A command can then be created and initialized with the device address and some data, and sent
//! the DAC.
//! ```rust, ignore
//! let mut dac_cmd = Command::default().address(0b111).data(14);
//! dac.send(dac_cmd);
//! ```
//!
//! New data can be sent using the existing command by just changing the data and re-sending.
//! ```rust, ignore
//! dac_cmd = dac_cmd.data(348);
//! dac.send(dac_cmd);
//! ```
//!
//! ## More information
//! - [MCP4725 datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf)
//! - [API documentation] (https://docs.rs/mcp4725/)
//! - [Github repository](https://github.com/mendelt/mcp4725)
//! - [Crates.io](https://crates.io/crates/mcp4725)
//!

#![no_std]

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

    /// Send a command to the MCP4725
    pub fn send(&mut self, command: &Command) {
        self.i2c.write(command.address_byte, &command.data_bytes());
    }
}

const DEVICE_ID: u8 = 0b1100;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PowerMode {
    Normal = 0b00,
    Resistor1kOhm = 0b01,
    Resistor100kOhm = 0b10,
    Resistor500kOhm = 0b11,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    address_byte: u8,
    write_eeprom: bool,
    power_mode: PowerMode,
    data: u16,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            address_byte: DEVICE_ID << 3,
            write_eeprom: false,
            power_mode: PowerMode::Normal,
            data: 0,
        }
    }
}

impl Command {
    /// Format data bytes to send to the DAC. At the moment only sending one sample at a time is
    /// supported.
    pub fn data_bytes(&self) -> [u8; 3] {
        [
            if self.write_eeprom == true {
                0x60
            } else {
                0x20
            } + (self.power_mode as u8)
                << 1, // TODO Should have extra parentheses, check on the scope
            (self.data >> 4) as u8,
            (self.data & 0x000f << 4) as u8,
        ]
    }

    /// Set the data to send with this command. This data will be truncated to a 12 bit int
    pub fn data(mut self, data: u16) -> Self {
        self.data = cmp::min(data, 0x0fff);
        self
    }

    /// Set the 3 bit address
    pub fn address(mut self, address: u8) -> Self {
        self.address_byte = (DEVICE_ID << 3) + (address & 0b00000111);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_address_with_device_id() {
        let cmd = Command::default().address(0b111);

        assert_eq!(cmd.address_byte, 0b01100111);
    }

    #[test]
    fn should_ignore_adresses_with_more_than_3_bits() {
        let cmd = Command::default().address(0b11111010);

        assert_eq!(cmd.address_byte, 0b01100010);
    }

    #[test]
    fn should_encode_data_into_data_bytes() {
        let cmd = Command::default().data(0x0fff);

        assert_eq!(cmd.data_bytes(), [0b01000000, 0b11111111, 0b11110000]) // TODO: check this on the scope
    }

    #[test]
    fn should_encode_command_into_data_bytes() {
        let cmd = Command::default().write_eeprom(true);

        assert_eq!(cmd.data_bytes(), [0b11000000, 0, 0]) // TODO: check this on the scope
    }
}
