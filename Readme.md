# CoolSnap

CoolSnap is a Linux utility that helps protect your system during shutdown. When a poweroff, reboot, or shutdown request is issued, CoolSnap automatically checks the CPU temperature and delays the shutdown process until the processor cools to a user-defined safe temperature.

## Features

- Automatically activates on poweroff, reboot, and shutdown requests.
- Monitors CPU temperature before shutdown.
- Allows a custom safe temperature threshold.
- Valid temperature range: **40°C–80°C**.
- Default safe temperature: **50°C**.
- Automatic fallback shutdown after **10 minutes**, even if the target temperature is not reached.
- Simple command-line interface.
- Available in Bash and Rust implementations.

## Usage

Set your desired safe shutdown temperature:

```bash
coolsnap <temperature>
```

Example:

```bash
coolsnap 45
```

The temperature must be between **40°C and 80°C**.

If no custom value is configured, CoolSnap uses the default temperature of **50°C**.

## Installation

### Option 1: Install the Bash Version

Run the installer:

```bash
chmod +x install.sh
sudo ./install.sh
```

### Option 2: Build and Install the Rust Version

If Rust is not installed, install it first:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then build and install CoolSnap:

```bash
chmod +x build.sh
./build.sh
```

The build script will automatically:

- Build CoolSnap from source
- Install it on your system
- Configure automatic shutdown integration

### Install an Already Compiled Rust Build

If you already have a compiled Rust binary, install it using:

```bash
sudo ./install rust
```

## Configuration

Change the safe temperature at any time:

```bash
sudo coolsnap <temperature>
```

Example:

```bash
sudo coolsnap 60
```

Requirements:

- Temperature must be between **40°C and 80°C**
- Values outside this range will be rejected

## How It Works

When a poweroff, reboot, or shutdown request is made:

1. CoolSnap intercepts the request.
2. The current CPU temperature is checked.
3. If the temperature is above the configured threshold, CoolSnap waits for the processor to cool down.
4. CPU temperature is monitored continuously.
5. Once the temperature reaches the configured safe level, the original shutdown or reboot operation proceeds.
6. If the target temperature is not reached within 10 minutes, CoolSnap performs the shutdown anyway as a fail-safe.

## Uninstallation

Remove CoolSnap from your system:

```bash
chmod +x uninstall.sh
sudo ./uninstall.sh
```

## Requirements

- Linux operating system
- CPU temperature sensors supported by the system
- Rust (only required when building the Rust version from source)

## Notes

- CoolSnap is intended to run automatically whenever the system is instructed to power off or reboot.
- The utility ensures the processor has an opportunity to cool before power loss.
- A built-in 10-minute timeout prevents indefinite waiting.
