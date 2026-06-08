#!/bin/bash

set -e

sudo systemctl disable coolsnap || true
sudo systemctl stop coolsnap || true

sudo rm -rf /opt/coolsnap
sudo rm -f /usr/local/bin/coolsnap
sudo rm -f /etc/systemd/system/coolsnap.service


echo -e "\n\n\e[32mUninstall successful\e[0m"