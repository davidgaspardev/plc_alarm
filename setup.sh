#!/bin/bash

# Check if libudev-dev is installed
dpkg -l | grep libudev-dev &> /dev/null

if [ $? -eq 0 ]; then
    echo "libudev-dev is already installed."
else
    echo "libudev-dev is not installed. Installing now..."
    sudo apt update
    sudo apt install -y libudev-dev
fi

read -p "Enter PLC ip to listener: " plc_address

service_path=/etc/systemd/system/plc_alarm.service
binary_path=/usr/local/bin/plc_alarm
working_directory="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

cd $working_directory

# Check if the binary file already exists
if [ -f $binary_path ]; then
    read -p "$binary_path already exists. Make a new build? (y/n): " choice
    if [[ $choice != "y" && $choice != "Y" ]]; then
        echo "Continue with build current."
    else
        cargo build --release
        sudo cp -f target/release/plc_alarm $binary_path
    fi
else
    cargo build --release
    sudo cp target/release/plc_alarm $binary_path
fi

# Check if the service file already exists
if [ -f $service_path ]; then
    read -p "$service_path already exists. Overwrite? (y/n): " choice
    if [[ $choice != "y" && $choice != "Y" ]]; then
        echo "Exiting without overwriting."
        exit 1
    fi
fi

# Create the service file content
cat <<EOL | sudo tee $service_path
[Unit]
Description=PLC Alarm for Tubarão Intelbras
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=/bin/bash -c '$binary_path $plc_address >> $working_directory/output.csv 2>> $working_directory/error.log'
Restart=always
User=root
Group=root
Environment=PATH=/usr/bin:/usr/local/bin
WorkingDirectory=$working_directory
# StandardOutput=file:$working_directory/output.csv
# StandardError=file:$working_directory/error.log
RestartSec=3
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
EOL

touch $working_directory/output.log
touch $working_directory/error.log

# Reload systemd
sudo systemctl daemon-reload

echo "Service file created and systemd reloaded."