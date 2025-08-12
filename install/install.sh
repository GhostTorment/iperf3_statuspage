#!/bin/bash

set -e

# Ensure script is run as root
if [[ $EUID -ne 0 ]]; then
    echo "❌ This script must be run as root."
    exit 1
fi

cd "$(dirname "$0")"

echo "🔧 Installing iperf3-statuspage service..."

# Copy files from package into the root filesystem
echo "📁 Copying files to system directories..."
cp -r usr/* /usr/
cp -r etc/* /etc/

# Set correct permissions
echo "🔐 Setting permissions..."
chmod 755 /usr/local/bin/iperf3-statuspage
chmod 644 /etc/iperf3-statuspage/.env
chmod 644 /etc/systemd/system/iperf3-statuspage.service

# Reload systemd and enable/start the service
echo "🔄 Reloading systemd..."
systemctl daemon-reload

echo "✅ Enabling and starting the service..."
systemctl enable --now iperf3-statuspage.service

echo "🚀 Service installed and running!"
systemctl status iperf3-statuspage.service --no-pager