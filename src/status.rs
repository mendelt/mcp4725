use crate::PowerDown;
use core::fmt::Debug;

/// The status of the MCP4725 as read by the read command. Contains the DAC register values and the
/// values stored in EEPROM
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacStatus {
    bytes: [u8; 5],
}

impl From<[u8; 5]> for DacStatus {
    fn from(bytes: [u8; 5]) -> Self {
        Self { bytes }
    }
}

impl Debug for DacStatus {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Point")
            .field("power_down", &self.power_down())
            .field("data", &self.data())
            .field("por", &self.por())
            .field("eeprom_write_status", &self.eeprom_write_status())
            .field("eeprom_data", &self.eeprom_data())
            .field("eeprom_power_down", &self.eeprom_power_down())
            .finish()
    }
}

impl DacStatus {
    /// Eeprom write status. true = completed, false = incomplete
    pub fn eeprom_write_status(&self) -> bool {
        self.bytes[0] & 0x80 == 0x80
    }

    /// Power on reset state
    pub fn por(&self) -> bool {
        self.bytes[0] & 0x40 == 0x40
    }

    /// Current power mode setting
    pub fn power_down(&self) -> PowerDown {
        // Should never fail. This distills a two bit value from bytes, PowerMode is defined
        // for each of the four possible values.
        ((self.bytes[0] & 0b00000110) >> 1).into()
    }

    /// Data currently stored in the DAC register
    pub fn data(&self) -> u16 {
        (self.bytes[1] as u16 * 0x0100 + self.bytes[2] as u16) >> 4
    }

    /// Power mode stored in eeprom
    pub fn eeprom_power_down(&self) -> PowerDown {
        // Should never fail. This distills a two bit value from bytes, PowerMode is defined
        // for each of the four possible values.
        ((self.bytes[3] & 0b01100000) >> 5).into()
    }

    /// Data stored in eeprom
    pub fn eeprom_data(&self) -> u16 {
        (self.bytes[3] & 0x0f) as u16 * 0x0100 + self.bytes[4] as u16
    }
}

#[cfg(test)]
mod test_status {
    use super::*;

    #[test]
    fn should_parse_eeprom_write_status() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.eeprom_write_status(), false);

        let status: DacStatus = [0xffu8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.eeprom_write_status(), true);
    }

    #[test]
    fn should_parse_dac_por() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.por(), false);

        let status: DacStatus = [0x40u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.por(), true);
    }

    #[test]
    fn should_parse_dac_data() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.data(), 0x0000);

        let status: DacStatus = [0u8, 0xffu8, 0xffu8, 0x0f0u8, 0u8].into();
        assert_eq!(status.data(), 0x0fff);
    }

    #[test]
    fn should_parse_eeprom_data() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.eeprom_data(), 0x0000);

        let status: DacStatus = [0u8, 0u8, 0u8, 0xffu8, 0xffu8].into();
        assert_eq!(status.eeprom_data(), 0x0fff);
    }

    #[test]
    fn should_parse_dac_power_down() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.power_down(), PowerDown::Normal);

        let status: DacStatus = [0b00000100u8, 0u8, 0u8, 0xffu8, 0xffu8].into();
        assert_eq!(status.power_down(), PowerDown::Resistor100kOhm);
    }

    #[test]
    fn should_parse_eeprom_power() {
        let status: DacStatus = [0u8, 0u8, 0u8, 0u8, 0u8].into();
        assert_eq!(status.eeprom_power_down(), PowerDown::Normal);

        let status: DacStatus = [0u8, 0u8, 0u8, 0xffu8, 0xffu8].into();
        assert_eq!(status.eeprom_power_down(), PowerDown::Resistor500kOhm);
    }
}
