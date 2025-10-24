#!/bin/bash

# Copy desktop file to applications directory
mkdir -p ~/.local/share/applications
cp cosmic-applet-asdcontrol.desktop ~/.local/share/applications/

echo "Installation complete! Add the applet through COSMIC Settings > Panel > Applets"
