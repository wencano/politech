# Cursor Cloud Agent environment — Ubuntu 24.04 + Docker Compose (DinD)
# Matches Dokploy production host; used to build/test dokploy-compose.yaml

FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        gnupg \
    && install -m 0755 -d /etc/apt/keyrings \
    && . /etc/os-release \
    && curl -fsSL "https://download.docker.com/linux/${ID}/gpg" -o /etc/apt/keyrings/docker.asc \
    && chmod a+r /etc/apt/keyrings/docker.asc \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/${ID} ${VERSION_CODENAME} stable" \
        > /etc/apt/sources.list.d/docker.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        docker-buildx-plugin \
        docker-ce \
        docker-ce-cli \
        docker-compose-plugin \
        fuse-overlayfs \
        git \
        git-lfs \
        iptables \
        less \
        nodejs \
        npm \
        pkg-config \
        ripgrep \
        sudo \
        tmux \
        unzip \
        wget \
    && rm -rf /var/lib/apt/lists/*

RUN usermod -aG docker ubuntu \
    && echo "ubuntu ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER ubuntu
WORKDIR /home/ubuntu

CMD ["/bin/bash"]
