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
//! ```rust, ignore
//! let mut dac = MCP4725::new(i2c, 0b010);
//! ```
//!
//! To set the dac output and powermode the dac register can be set;
//! ```rust, ignore
//! dac.set_dac(PowerDown::Normal, 0x0fff);
//! ```
//!
//! The MCP4725 has a built in eeprom that is used to initialize the dac register on power up.
//! The values in the eeprom can be set with the `set_dac_and_eeprom` method;
//! ```rust, ignore
//! dac.set_dac_and_eeprom(PowerDown::Resistor100kOhm, 0x0fff);
//! ```
//!
//! ## More information
//! - [MCP4725 datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf)
//! - [API documentation](https://docs.rs/mcp4725/)
//! - [Github repository](https://github.com/mendelt/mcp4725)
//! - [Crates.io](https://crates.io/crates/mcp4725)
//!
//! ## Todo
//! [] Create an example writing eeprom
//! [] Implement sending multiple consecutive fast commands
#![no_std]

mod encode;
mod status;

use core::fmt::Debug;
use embedded_hal::blocking::i2c::{Read, Write};
use encode::{encode_address, encode_command, encode_fast_command};
pub use status::DacStatus;

#[warn(missing_debug_implementations, missing_docs)]

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

    /// Send a command to the MCP4725
    pub fn send(&mut self, command: &Command) -> Result<(), E> {
        self.i2c.write(self.address, &command.bytes())?;
        Ok(())
    }

    /// Send a fast command to the MCP4725
    pub fn send_fast(&mut self, command: &FastCommand) -> Result<(), E> {
        self.i2c.write(self.address, &command.bytes())?;
        Ok(())
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

impl Command {
    /// Format data bytes to send to the DAC. At the moment only sending one sample at a time is
    /// supported.
    pub fn bytes(&self) -> [u8; 3] {
        [self.command_byte, self.data_byte_0, self.data_byte_1]
    }

    /// Set the data to send with this command. This data will be truncated to a 12 bit int
    pub fn data(mut self, data: u16) -> Self {
        self.data_byte_0 = (data >> 4) as u8;
        self.data_byte_1 = (data & 0x000f << 4) as u8;

        self
    }

    /// Write the supplied values to the EEPROM as well as to the DAC
    pub fn command_type(mut self, command: CommandType) -> Self {
        self.command_byte = (self.command_byte & 0b00011111) | command as u8;
        self
    }

    /// Set the power mode
    pub fn power_mode(mut self, mode: PowerDown) -> Self {
        self.command_byte = (self.command_byte & 0b11111000) | ((mode as u8) << 1);
        self
    }
}

#[cfg(test)]
mod test_command {
    use super::*;

    #[test]
    fn should_encode_data_into_data_bytes() {
        let cmd = Command::default().data(0x0fff);

        assert_eq!(cmd.bytes(), [0b01000000, 0b11111111, 0b11110000])
    }

    #[test]
    fn should_encode_power_mode_into_data_bytes() {
        let cmd = Command::default().power_mode(PowerDown::Resistor1kOhm);

        assert_eq!(cmd.bytes(), [0b01000010, 0, 0])
    }

    #[test]
    fn should_encode_command_into_data_bytes() {
        let cmd = Command::default().command_type(CommandType::WriteDacAndEEPROM);

        assert_eq!(cmd.bytes(), [0b01100000, 0, 0])
    }
}

/// A FastCommand to send to the MCP4725.
/// Fast commands are stripped down commands that can be used to send data in only 2 bytes instead
/// of 3. It can only be used to set the DAC register. Not to write to the EEPROM that stores the
/// default values.
/// As with the normal Command the address(), power_mode() and data() builder methods can be used to
/// set parameters. FastCommands can be sent using the send_fast method on the MCP4725 driver.
pub struct FastCommand {
    data_byte_0: u8,
    data_byte_1: u8,

    powermode: u8,
}

impl Default for FastCommand {
    fn default() -> Self {
        FastCommand {
            powermode: 0,
            data_byte_0: 0,
            data_byte_1: 0,
        }
    }
}

impl FastCommand {
    pub fn bytes(&self) -> [u8; 2] {
        [self.data_byte_0, self.data_byte_1]
    }

    /// Set the data to send with this command. This data will be truncated to a 12 bit int
    pub fn data(mut self, data: u16) -> Self {
        self.data_byte_0 = ((data >> 8) as u8) | self.powermode;
        self.data_byte_1 = data as u8;

        self
    }

    /// Set the power mode
    pub fn power_mode(mut self, mode: PowerDown) -> Self {
        self.powermode = (mode as u8) << 4;
        self.data_byte_0 = (self.data_byte_0 & 0x0f) | self.powermode;

        self
    }
}

#[cfg(test)]
mod test_fast_command {
    use super::*;

    #[test]
    fn should_encode_data_into_data_bytes() {
        let cmd = FastCommand::default().data(0x0fff);

        assert_eq!(cmd.bytes(), [0b00001111, 0b11111111])
    }

    #[test]
    fn should_encode_powermode_into_data_bytes() {
        let cmd = FastCommand::default().power_mode(PowerDown::Resistor500kOhm);

        assert_eq!(cmd.bytes(), [0b00110000, 0])
    }
}
