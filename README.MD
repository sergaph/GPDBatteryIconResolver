# GPD Battery Icon Resolver

A utility tool designed to fix the disappearing battery icon issue in the Windows taskbar for GPD devices after updating graphics card drivers.

## Problem Description

After updating graphics card drivers on GPD devices, the battery icon may disappear from the Windows taskbar. This utility resolves this issue by:

1. Refreshing the hardware configuration in Device Manager
2. Restarting Windows Explorer to apply the changes

## Building

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- Windows SDK (for building Windows-specific code)

### Build Instructions

1. Clone the repository:
   ```bash
   git clone https://github.com/sergaph/GPDBatteryIconResolver.git
   cd WinBatteryIconChecker
   ```

2. Build the release version:
   ```bash
   cargo build --release
   ```

3. The executable will be located at:
   ```
   target/release/GPDBatteryIconResolver.exe
   ```

### Development Build

For a debug build (faster compilation, slower execution):
```bash
cargo build
```

The debug executable will be at:
```
target/debug/GPDBatteryIconResolver.exe
```

## Usage

1. **Run as Administrator**: Right-click the executable and select "Run as administrator" (required for hardware configuration changes)

2. The utility will:
   - Refresh hardware configuration in Device Manager
   - Wait 3 seconds
   - Restart Windows Explorer
   - Wait 2 seconds for Explorer to fully restart
   - Display completion messages

3. After completion, check if the battery icon has reappeared in the taskbar.

## Requirements

- Windows operating system
- Administrator privileges (required for device configuration changes)

## License

Copyright (c) 2025

