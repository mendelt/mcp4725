//! *Microchip MCP4725 DAC Driver for Rust Embedded HAL*
//! This is a driver crate for embedded Rust. It's built on top of the Rust
//! [embedded HAL](https://github.com/rust-embedded/embedded-hal)
//! It supports sending commands to a MCP4725 DAC over I2C.
//! To get started you can look at the
//! [examples](https://github.com/mendelt/mcp4725/tree/master/bluepill-examples/examples)
//! on how to use this driver on an inexpensive blue pill STM32F103 board.
//!
//! The driver can be initialized by calling create and passing it an I2C interface. The three least
//! significant bits of the device address (A2, A1 and A0) also need to be specified. A2 and A1 are
//! set in the device. A0 can be set by pulling the corresponding connection on the device high or
//! low.
//! ```
//! # use embedded_hal_mock::i2c::Mock;
//! # use mcp4725::*;
//! # let mut i2c = Mock::new(&[]);
//! let mut dac = MCP4725::new(i2c, 0b010);
//! ```
//!
//! To set the dac output and powermode the dac register can be set;
//! ```
//! # use embedded_hal_mock::i2c::{Mock, Transaction};
//! # use mcp4725::*;
//! # let mut i2c = Mock::new(&[Transaction::write(98, vec![0x40, 0xff, 0xf0]),]);
//! # let mut dac = MCP4725::new(i2c, 0b010);
//! dac.set_dac(PowerDown::Normal, 0x0fff);
//! ```
//!
//! The MCP4725 has a built in eeprom that is used to initialize the dac register on power up.
//! The values in the eeprom can be set with the `set_dac_and_eeprom` method;
//! ```
//! # use embedded_hal_mock::i2c::{Mock, Transaction};
//! # use mcp4725::*;
//! # let mut i2c = Mock::new(&[Transaction::write(98, vec![0x64, 0xff, 0xf0])]);
//! # let mut dac = MCP4725::new(i2c, 0b010);
//! dac.set_dac_and_eeprom(PowerDown::Resistor100kOhm, 0x0fff);
//! ```
//!
//! ## More information
//! - [MCP4725 datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf)
//! - [API documentation](https://docs.rs/mcp4725/)
//! - [Github repository](https://github.com/mendelt/mcp4725)
//! - [Crates.io](https://crates.io/crates/mcp4725)
//!
#![no_std]
#![warn(missing_debug_implementations, missing_docs)]

mod encode;
mod status;

use core::fmt::Debug;
use embedded_hal::blocking::i2c::{Read, Write};
use encode::{encode_address, encode_command, encode_fast_command};
pub use status::DacStatus;


/// MCP4725 DAC driver. Wraps an I2C port to send commands to an MCP4725
#[derive(Debug)]
pub struct MCP4725<I2C>
where
    I2C: Read + Write,
{
    i2c: I2C,
    address: u8,
}

impl<I2C, E> MCP4725<I2C>
where
    I2C: Read<Error = E> + Write<Error = E>,
{
    /// Construct a new MCP4725 driver instance.
    /// i2c is the initialized i2c driver port to use,
    /// user_address is the three bit user-part of the i2c address where the MCP4725 can be reached
    ///   - The least significant bit of this address can be set externally by pulling the A0 leg of
    ///     the chip low (0) or high (1)
    ///   The two most significant bits are set in the factory. There are four variants of the chip
    ///     with different addresses.
    pub fn new(i2c: I2C, user_address: u8) -> Self {
        MCP4725 {
            i2c,
            address: encode_address(user_address),
        }
    }

    /// Set the dac register
    pub fn set_dac(&mut self, power: PowerDown, data: u16) -> Result<(), E> {
        let bytes = encode_command(CommandType::WriteDac, power, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Set the dac and eeprom registers
    pub fn set_dac_and_eeprom(&mut self, power: PowerDown, data: u16) -> Result<(), E> {
        let bytes = encode_command(CommandType::WriteDacAndEEPROM, power, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Use the two byte fast command to set the dac register
    pub fn set_dac_fast(&mut self, power: PowerDown, data: u16) -> Result<(), E> {
        let bytes = encode_fast_command(power, data);
        self.i2c.write(self.address, &bytes)
    }

    /// Send read command and return the dac status
    pub fn read(&mut self) -> Result<DacStatus, E> {
        let mut buffer: [u8; 5] = [0; 5];
        self.i2c.read(self.address, &mut buffer)?;

        Ok(buffer.into())
    }

    /// Send a wake-up command over the I2C bus.
    /// WARNING: This is a general call command and can wake-up other devices on the bus as well.
    pub fn wake_up(&mut self) -> Result<(), E> {
        self.i2c.write(0x00, &[0x06u8])?;
        Ok(())
    }

    /// Send a reset command on the I2C bus.
    /// WARNING: This is a general call command and can reset other devices on the bus as well.
    pub fn reset(&mut self) -> Result<(), E> {
        self.i2c.write(0x00, &[0x09u8])?;
        Ok(())
    }

    /// Destroy the MCP4725 driver, return the wrapped I2C
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

/// Two bit flags indicating the power down mode for the MCP4725
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PowerDown {
    Normal = 0b00,
    Resistor1kOhm = 0b01,
    Resistor100kOhm = 0b10,
    Resistor500kOhm = 0b11,
}

impl From<u8> for PowerDown {
    fn from(mode: u8) -> Self {
        match mode {
            0b00 => PowerDown::Normal,
            0b01 => PowerDown::Resistor1kOhm,
            0b10 => PowerDown::Resistor100kOhm,
            0b11 => PowerDown::Resistor500kOhm,
            _ => panic!("Invalid powerdown value"),
        }
    }
}

/// The type of the command to send for a Command
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum CommandType {
    WriteDac = 0x40,
    WriteDacAndEEPROM = 0x60,
}

/// A Command to send to the MCP4725.
/// Using the address(), command_type(), power_mode() and data() builder methods the
/// parameters for this command can be set. Commands can be sent using the send method on the
/// MCP4725 driver.
/// A command can (and should) be re-used. data() can be used to re-set the data while keeping other
/// parameters the same.
#[derive(Debug, Eq, PartialEq)]
pub struct Command {
    command_byte: u8,
    data_byte_0: u8,
    data_byte_1: u8,
}

impl Default for Command {
    /// Instantiate a command with sane defaults.
    fn default() -> Self {
        Self {
            command_byte: CommandType::WriteDac as u8,
            data_byte_0: 0,
            data_byte_1: 0,
        }
    }
}
