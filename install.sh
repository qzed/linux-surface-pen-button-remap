#!/bin/bash

. /etc/os-release
if [ "$ID" == "fedora" ]; then
	echo "Installing dependencies..."
	sudo dnf install cargo libevdev-devel -y
elif [ "$ID" == "ubuntu" ] || [ "$ID" == "debian" ]; then
	echo "Installing dependendies..."
	sudo apt install cargo libevdev-dev -y
else
	echo "You have to install the packages cargo and libevdev-dev/libevdev-devel to use this program. Please do that before continuing"
	read -p "Press Enter to continue" </dev/tty
fi

cargo build --release
sudo killall surface-pen-button
sudo cp ./target/release/surface-pen-button /usr/bin/

printf 'Do you want to install the systemd service (y/n)? '
read answer

if [ "$answer" != "${answer#[Yy]}" ]; then
	sudo cp surface-pen-button.service /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable --now surface-pen-button.service
	echo "Systemd service is installed, enabled and started!"
else
	echo "Systemd service is not installed! Run this tool via 'sudo surface-pen-button'"
fi
