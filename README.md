# mcp4725 &emsp; [![Build Status](https://travis-ci.com/mendelt/mcp4725.svg?branch=master)](https://travis-ci.org/mendelt/mcp4725)

*Microchip MCP4725 DAC Driver for Rust Embedded HAL*
This is a driver crate for embedded Rust. It's built on top of the Rust
[embedded HAL](https://github.com/rust-embedded/embedded-hal)
It supports sending commands to a MCP4725 DAC over I2C.
To get started you can look at the
[examples](https://github.com/mendelt/mcp4725/tree/master/examples)
on how to use this driver on an inexpensive blue pill STM32F103 board.

The driver can be initialized by calling create and passing it an I2C interface.
```rust, ignore
let mut dac = MCP4725::create(i2c);
```

A command can then be created and initialized with the device address and some data, and sent
the DAC.
```rust, ignore
let mut dac_cmd = Command::default().address(0b111).data(14);
dac.send(dac_cmd);
```

New data can be sent using the existing command by just changing the data and re-sending.
```rust, ignore
dac_cmd = dac_cmd.data(348);
dac.send(dac_cmd);
```

## More information
- [MCP4725 datasheet](http://ww1.microchip.com/downloads/en/DeviceDoc/22039d.pdf)
- [API documentation](https://docs.rs/mcp4725/)
- [Github repository](https://github.com/mendelt/mcp4725)
- [Crates.io](https://crates.io/crates/mcp4725)

## Todo
[] Move address to driver, make a0 to a2 configurable
[] Implement read command
[] Implement sending multiple fast commands to the same address
[] Implement general call reset and wake-up if needed
[] Possibly implement high speed mode

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
