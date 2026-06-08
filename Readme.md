# CoolSnap

CoolSnap is a Linux-based utility that helps protect your system during shutdown. Before powering off, it checks your CPU temperature and waits until it cools down to a user-defined safe temperature.

## Features

- Monitors CPU temperature before shutdown.
- Allows you to set a custom safe temperature threshold.
- Valid temperature range: 40°C–80°C.
- Default safe temperature: 50°C.
- Automatic fallback shutdown after 10 minutes, even if the CPU temperature remains above the target threshold.
- Simple command-line interface.

# Usage

Set your desired safe shutdown temperature:

```bash
coolsnap <temperature>
```

Example:

```bash
coolsnap 45
```

The temperature must be between 40°C and 80°C.

If no custom value is configured, CoolSnap uses the default temperature of 50°C.

## Installation

For x86_64 Linux Systems

Run the installer:

```bash
chmod +x install.sh
./install.sh
```

## For Other Architectures

First, install Rust:

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

## Configuration

Change the safe temperature at any time:

```bash
coolsnap <temperature>
```

Example:

```bash
coolsnap 60
```

Requirements:

Temperature must be between 40°C and 80°C
Values outside this range will be rejected

## How It Works

When a shutdown is requested:

- CoolSnap checks the current CPU temperature.
- If the temperature is above your configured threshold, it waits for the CPU to cool down.
- Once the temperature reaches the safe level,the system shuts down.
- If the temperature does not reach the target within 10 minutes, CoolSnap performs the shutdown anyway as a fail-safe.

## Uninstallation

Remove CoolSnap from your system:

```bash
chmod +x uninstall.sh
./uninstall.sh
```

# Requirements

- Linux operating system
- CPU temperature sensors supported by the system
- Rust (only required when building from source)
