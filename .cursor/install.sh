#!/bin/bash
# Runs once inside the Cursor cloud agent container (idempotent; snapshotted after).

set -euxo pipefail

sudo mkdir -p /etc/docker
sudo tee /etc/docker/daemon.json > /dev/null <<'EOF'
{
  "storage-driver": "fuse-overlayfs",
  "iptables": false
}
EOF

# dokploy-compose.yaml expects this external network
docker network create dokploy-network 2>/dev/null || true

npm install
