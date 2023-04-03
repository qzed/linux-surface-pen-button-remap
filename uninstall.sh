#!/bin/bash

sudo killall surface-pen-button
sudo rm -r /etc/surface-pen-button/
sudo rm -r /usr/bin/surface-pen-button

SYSTEMD_FILE=/etc/systemd/system/surface-pen-button.service

if test -f "$SYSTEMD_FILE"; then
	sudo systemctl stop surface-pen-button.service
	sudo rm -r /etc/systemd/system/surface-pen-button.service
	sudo systemctl daemon-reload
	echo "Finished!"
else
	echo "Finished!"
fi
