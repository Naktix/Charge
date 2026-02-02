#!/bin/bash
set -e

echo "Uninstalling Charge"

sudo rm -f /usr/bin/charge
sudo rm -f /usr/share/applications/dev.naktix.Charge.desktop
sudo rm -f /usr/share/metainfo/dev.naktix.Charge.metainfo.xml

echo "Please restart COSMIC or log out/in"
echo ""