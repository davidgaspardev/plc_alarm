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

SERVICE_PATH=/etc/systemd/system/plc_alarm.service
BINARY_PATH=/usr/local/bin/plc_alarm
WORK_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Change directory to work directory
cd $WORK_DIR

# Check if the binary file already exists
if [ -f $BINARY_PATH ]; then
    read -p "$BINARY_PATH already exists. Make a new build? (y/n): " choice
    if [[ $choice != "y" && $choice != "Y" ]]; then
        echo "Continue with build current."
    else
        cargo build --release
        sudo cp -f target/release/plc_alarm $BINARY_PATH
    fi
else
    cargo build --release
    sudo cp target/release/plc_alarm $BINARY_PATH
fi

# Check if the service file already exists
if [ -f $SERVICE_PATH ]; then
    read -p "$SERVICE_PATH already exists. Overwrite? (y/n): " choice
    if [[ $choice != "y" && $choice != "Y" ]]; then
        echo "Exiting without overwriting."
        exit 1
    fi
fi

read -p "Enter PLC ip to listener: " PLC_ADDRESS

# Create the service file content
cat <<EOL | sudo tee $SERVICE_PATH
[Unit]
Description=PLC Alarm for Tubarão Intelbras
After=network-online.target
Wants=network-online.target

[Service]
ExecStart=/bin/bash -c '$BINARY_PATH $PLC_ADDRESS >> $WORK_DIR/output.csv 2>> $WORK_DIR/error.log'
Restart=always
User=root
Group=root
Environment=PATH=/usr/bin:/usr/local/bin
WorkingDirectory=$WORK_DIR
# StandardOutput=file:$WORK_DIR/output.csv
# StandardError=file:$WORK_DIR/error.log
RestartSec=3
LimitNOFILE=4096

[Install]
WantedBy=multi-user.target
EOL

touch $WORK_DIR/output.log
touch $WORK_DIR/error.log

# Reload systemd
sudo systemctl daemon-reload

echo "Service file created and systemd reloaded."