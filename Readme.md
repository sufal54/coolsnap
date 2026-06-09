# CoolSnap

CoolSnap is a Linux utility that helps protect your system during shutdown. Before powering off, it monitors CPU temperature and waits until the processor cools down to a user-defined safe temperature.

## Features

- Monitors CPU temperature before shutdown.
- Allows a custom safe temperature threshold.
- Valid temperature range: **40°C–80°C**.
- Default safe temperature: **50°C**.
- Automatic fallback shutdown after **10 minutes**, even if the target temperature is not reached.
- Simple command-line interface.

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
- Perform the required setup

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

When a shutdown is requested:

1. CoolSnap checks the current CPU temperature.
2. If the temperature is above the configured threshold, it waits for the CPU to cool down.
3. Once the temperature reaches the safe level, the system shuts down.
4. If the target temperature is not reached within 10 minutes, CoolSnap shuts down the system anyway as a fail-safe.

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
