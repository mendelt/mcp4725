//! *Microchip MCP4725 DAC Driver for Rust Embedded HAL*
//! This is a driver crate for embedded Rust. It's built on top of the Rust
//! [embedded HAL](https://github.com/rust-embedded/embedded-hal)
//! It supports sending commands to a MCP4725 DAC over I2C.
//! To get started you can look at the
//! [examples](https://github.com/mendelt/mcp4725/tree/master/bluepill-examples/examples)
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
//! - [API documentation](https://docs.rs/mcp4725/)
//! - [Github repository](https://github.com/mendelt/mcp4725)
//! - [Crates.io](https://crates.io/crates/mcp4725)
//!
//! ## Todo
//! [] Move address to driver, make a0 to a2 configurable
//! [] Implement read command
//! [] Implement sending multiple fast commands to the same address
//! [] Implement general call reset and wake-up if needed
//! [] Possibly implement high speed mode

#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::blocking::i2c::Write;

const DEVICE_ID: u8 = 0b1100000;

fn encode_address(user_address: u8) -> u8 {
    DEVICE_ID | (user_address & 0b00000111)
}

#[cfg(test)]
mod test_address {
    use super::*;

    #[test]
    fn should_encode_address_with_device_id() {
        assert_eq!(encode_address(0b111), 0b01100111);
    }

    #[test]
    fn should_ignore_extra_user_bits() {
        let addr = encode_address(0b11111010);
        assert_eq!(addr, 0b01100010);
    }
}

/// MCP4725 DAC driver. Wraps an I2C port to send commands to an MCP4725
pub struct MCP4725<I2C>
where
    I2C: Write,
{
    i2c: I2C,
    address: u8,
}

impl<I2C> MCP4725<I2C>
where
    I2C: Write,
{
    pub fn new(i2c: I2C, user_address: u8) -> Self {
        MCP4725 {
            i2c: i2c,
            address: encode_address(user_address),
        }
    }

    /// Send a command to the MCP4725
    pub fn send(&mut self, command: &Command) {
        self.i2c.write(self.address, &command.bytes());
    }

    /// Send a fast command to the MCP4725
    pub fn send_fast(&mut self, command: &FastCommand) {
        self.i2c.write(self.address, &command.bytes());
    }
}

/// Two bit flags indicating the power mode for the MCP4725
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum PowerMode {
    Normal = 0b00,
    Resistor1kOhm = 0b01,
    Resistor100kOhm = 0b10,
    Resistor500kOhm = 0b11,
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
    pub fn power_mode(mut self, mode: PowerMode) -> Self {
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
        let cmd = Command::default().power_mode(PowerMode::Resistor1kOhm);

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
    pub fn power_mode(mut self, mode: PowerMode) -> Self {
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
        let cmd = FastCommand::default().power_mode(PowerMode::Resistor500kOhm);

        assert_eq!(cmd.bytes(), [0b00110000, 0])
    }
}
