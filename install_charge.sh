#!/bin/bash
set -e

echo "Installing Charge"

cargo build --release
sudo install -Dm755 target/release/charge /usr/bin/charge
sudo install -Dm644 dev.naktix.Charge.desktop /usr/share/applications/dev.naktix.Charge.desktop
sudo install -Dm644 dev.naktix.Charge.metainfo.xml /usr/share/metainfo/dev.naktix.Charge.metainfo.xml

echo "Please restart COSMIC or log out/in"
echo ""