# blesd

[![License](https://img.shields.io/github/license/petrzelenka/blesd)](LICENSE)
[![Latest release](https://img.shields.io/github/v/release/petrzelenka/blesd)](releases)

Blesd is a command-line tool for discovering services and characteristics provided by Bluetooth LE peripheral devices.

## Installation

[Releases](releases) contain precompiled binaries for Windows (x86_64) and macOS (AArch64).
Just extract the binary from the respective archive and place it on `PATH`.

## Usage

To scan for peripheral devices, use the `blesd scan` command (for detailed information about the command parameters, use the `blesd help scan` command):

![Scanning for devices on macOS](doc/macos_scan.png)

![Scanning for devices on Windows](doc/windows_scan.png)

To discover the services and characteristics provided by a peripheral device, use the `blesd discover <DEVICE_IDENTIFIER>` command (for detailed information about the command parameters, use the `blesd help discover` command):

![Discovering services and characteristics on macOS](doc/macos_discover.png)

![Discovering services and characteristics on Windows](doc/windows_discover.png)

NOTE: There are fundamental differences in handling of Bluetooth LE devices across the operating systems, resulting in different device identifiers being used and slightly different discovered services being reported.
Blesd works exactly according to the internal rules on each operating system.
