#!/bin/bash

# Install udev rules for Apple Studio Display and Apple Pro XDR Display
# This script must be run with sudo

set -e

if [ "$EUID" -ne 0 ]; then
    echo "This script must be run as root (use sudo)"
    exit 1
fi

echo "Installing udev rules for Apple displays..."

# Apple Studio Display (2022, 27")
echo "Creating rule for Apple Studio Display..."
cat > /etc/udev/rules.d/50-apple-studio.rules <<EOF
KERNEL=="hiddev*", ATTRS{idVendor}=="05ac", ATTRS{idProduct}=="1114", GROUP="users", OWNER="root", MODE="0660"
EOF
echo "✓ Created /etc/udev/rules.d/50-apple-studio.rules"

# Apple Pro XDR Display (2019, 32")
echo "Creating rule for Apple Pro XDR Display..."
cat > /etc/udev/rules.d/50-apple-xdr.rules <<EOF
KERNEL=="hiddev*", ATTRS{idVendor}=="05ac", ATTRS{idProduct}=="9243", GROUP="users", OWNER="root", MODE="0660"
EOF
echo "✓ Created /etc/udev/rules.d/50-apple-xdr.rules"

# Reload udev rules
echo "Reloading udev rules..."
udevadm control --reload-rules
udevadm trigger

echo ""
echo "✓ udev rules installed successfully!"
echo ""
echo "Note: You may need to log out and back in for the permissions to take effect."
echo ""
