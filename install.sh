cat > ~/.local/share/applications/asdcontrol-gnome.desktop << 'EOF'
[Desktop Entry]
Name=ASDControl
Comment=Control Apple Studio Display brightness
Exec=/home/snipesy/.local/bin/asdcontrol-gnome
Icon=preferences-desktop-display
Type=Application
Terminal=false
Categories=System;Settings;Hardware;
Keywords=brightness;display;monitor;apple;studio;
StartupNotify=true
EOF

chmod +x ~/.local/share/applications/asdcontrol-gnome.desktop
update-desktop-database ~/.local/share/applications/
