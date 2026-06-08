#!/bin/bash
set -e

cargo build --release

[ -x install.sh ] || chmod +x install.sh

sudo ./install.sh

echo -e "\n\n\e[32mBuild and install successful!\e[0m"