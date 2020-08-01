# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
...

### Changed
- Move to github actions for build
- Fix lint settings and fix resulting warnings about missing docs
- Clean up examples

## [0.4.0] - 2020-07-19

### Changed
- Remove deprecated API for sending commands
- Update dependencies to newer versions

## [0.3.1] - 2020-05-12

### Changed
- Un-deprecate structs to prevent unneeded warnings

## [0.3.0] - 2020-05-12

### Added
- Add an example showing how to read DAC and EEPROM status

### Changed
- Deprecate old-style API

## [0.2.2] - 2020-05-10

### Added
- Implement new simpler API for setting output and EEPROM registers instead of sending commands
- Update examples to use new API

### Changed
- Rename Power and PowerMode to PowerDown to be more consistent

## [0.2.1] - 2020-05-08

### Added
- Add method for reading and parsing dac and eeprom status

## [0.2.0] - 2020-04-20

### Added
- Add destroy method for destroying the driver and retrieving the wrapped I2C instance
- Add wake-up and reset general call commands
- Added example for sending fastcommands

### Changed
- Improved error handling
- Moved examples to separate bluepill-example project
- Documentation fixes

## [0.1.2] - 2019-06-09

### Changed
- Documentation fixes

## [0.1.1] - 2019-06-04

### Added
- Added fastcommand for setting output register with just two bytes
- Improved documentation

## [0.1.0] - 2019-05-08

### Added
- Send basic command to set output and power down mode
- Example for sending basic commands

[unreleased]: https://github.com/mendelt/mcp4725/compare/0.4.0...HEAD
[0.4.0]: https://github.com/mendelt/mcp4725/releases/tag/0.4.0
[0.3.1]: https://github.com/mendelt/mcp4725/releases/tag/0.3.1
[0.3.0]: https://github.com/mendelt/mcp4725/releases/tag/0.3.0
[0.2.2]: https://github.com/mendelt/mcp4725/releases/tag/0.2.2
[0.2.1]: https://github.com/mendelt/mcp4725/releases/tag/0.2.1
[0.2.0]: https://github.com/mendelt/mcp4725/releases/tag/0.2.0
[0.1.2]: https://github.com/mendelt/mcp4725/releases/tag/0.1.2
[0.1.1]: https://github.com/mendelt/mcp4725/releases/tag/0.1.1
[0.1.0]: https://github.com/mendelt/mcp4725/releases/tag/0.1.0
