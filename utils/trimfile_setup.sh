#!/bin/bash

WORK_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SERVICE_PATH=/etc/systemd/system/trimfile.service
TIMER_PATH=/etc/systemd/system/trimfile.timer

if [ -f $SERVICE_PATH ]; then
    read -p "$SERVICE_PATH already exists. Overwrite ? (y/n): " CHOICE
    if [[ $CHOICE != "y" && $CHOICE != "Y" ]]; then
        echo "Exiting without overwriting."
        exit 1
    fi
fi

# Create the service file content
cat <<EOL | sudo tee $SERVICE_PATH
[Unit]
Description=Trim file to ensure it doesn't exceed 12 mil lines

[Service]
Type=oneshot
ExecStart=$WORK_DIR/trimfile.sh
EOL

if [ -f $TIMER_PATH ]; then
    read -p "$TIMER_PATH already exists. Overwrite ? (y/n): " CHOICE
    if [[ $CHOICE != "y" && $CHOICE != "Y" ]]; then
        echo "Exiting without overwriting."
        exit 1
    fi
fi

# Create the service file content
cat <<EOL | sudo tee $SERVICE_PATH
[Unit]
[Unit]
Description=Run trimfile.service every hour

[Timer]
OnBootSec=5min
OnUnitActiveSec=1h
Unit=trimfile.service

[Install]
WantedBy=timers.target
EOL

# Reload systemd
sudo systemctl daemon-reload