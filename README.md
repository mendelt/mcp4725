# mcp4725 &emsp; [![Build Status](https://travis-ci.com/mendelt/mcp4725.svg?branch=master)](https://travis-ci.org/mendelt/mcp4725)

*Microchip MCP4725 DAC Driver for Rust Embedded HAL*
This is a driver crate for embedded Rust. It's built on top of the Rust
[embedded HAL](https://github.com/rust-embedded/embedded-hal)
It supports sending commands to a MCP4725 DAC over I2C.
To get started you can look at the
[examples](https://github.com/mendelt/mcp4725/tree/master/bluepill-examples/examples)
on how to use this driver on an inexpensive blue pill STM32F103 board.

The driver can be initialized by calling create and passing it an I2C interface. The three least
significant bits of the device address (A2, A1 and A0) also need to be specified. A2 and A1 are
set in the device. A0 can be set by pulling the corresponding connection on the device high or
low.
```rust, ignore
let mut dac = MCP4725::new(i2c, 0b010);
```

To set the dac output and powermode the dac register can be set;
```rust, ignore
dac.set_dac(PowerDown::Normal, 0x0fff);
```

The MCP4725 also has a built in eeprom to store the value of the dac register needs to have the
next time the device is powered on;
```rust, ignore
dac.set_dac_and_eeprom(PowerDown::Resistor100kOhm, 0x0fff);
```

## More information
- [MCP4725 datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf)
- [API documentation](https://docs.rs/mcp4725/)
- [Github repository](https://github.com/mendelt/mcp4725)
- [Crates.io](https://crates.io/crates/mcp4725)

## Todo
[] Create an example writing eeprom
[] Implement sending multiple consecutive fast commands

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
