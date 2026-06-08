#!/bin/bash

set -e

[ -x ./target/release/installer ] || chmod +x ./target/release/installer

./target/release/installer

chmod +x /opt/coolsnap/bin/coolsnap
chmod +x /opt/coolsnap/bin/temptool.sh

sudo systemctl enable coolsnap
sudo systemctl start coolsnap

echo -e "\n\n\e[32mInstall successful\e[0m"