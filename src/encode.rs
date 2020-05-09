//! Functions for encoding messages to send to the MCP4725

use crate::{CommandType, PowerDown};

/// The device id part of the MCP4725 address
const DEVICE_ID: u8 = 0b1100000;

/// Use the MCP4725 device id and the three bit user_address to encode the complete DAC address
pub fn encode_address(user_address: u8) -> u8 {
    DEVICE_ID | (user_address & 0b00000111)
}

/// Encode command type, powerdown mode and data into a three byte command
pub fn encode_command(command: CommandType, power: PowerDown, data: u16) -> [u8; 3] {
    [
        command as u8 + ((power as u8) << 1),
        (data >> 4) as u8,
        (data & 0x000f << 4) as u8,
    ]
}

#[cfg(test)]
mod test {
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

    #[test]
    fn should_encode_command_data() {
        let bytes = encode_command(CommandType::WriteDac, PowerDown::Normal, 0x0fff);

        assert_eq!(bytes, [0b01000000, 0b11111111, 0b11110000])
    }

    #[test]
    fn should_not_encode_command_data_over_12bits() {
        let bytes = encode_command(CommandType::WriteDac, PowerDown::Normal, 0xffff);

        assert_eq!(bytes, [0b01000000, 0b11111111, 0b11110000])
    }

    #[test]
    fn should_encode_power_mode() {
        let bytes = encode_command(CommandType::WriteDac, PowerDown::Resistor1kOhm, 0);

        assert_eq!(bytes, [0b01000010, 0, 0])
    }

    #[test]
    fn should_encode_command_type_into_data_bytes() {
        let bytes = encode_command(CommandType::WriteDacAndEEPROM, PowerDown::Normal, 0);

        assert_eq!(bytes, [0b01100000, 0, 0])
    }
}
