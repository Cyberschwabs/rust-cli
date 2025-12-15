#!/usr/bin/env bash

# Function for a simple progress spinner
spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while kill -0 $pid 2>/dev/null; do
        for i in $(seq 0 3); do
            printf "\r${spinstr:$i:1} $2"
            sleep $delay
        done
    done
    printf "\r$2... Done! ✅\n"
}

# Colors
GREEN="\e[32m"
YELLOW="\e[33m"
CYAN="\e[36m"
MAGENTA="\e[35m"
RESET="\e[0m"

RUSTUP_PATH="$HOME/.cargo/bin/rustup"

# Check if rustup is installed
if [ -x "$RUSTUP_PATH" ]; then
    echo -e "${GREEN}✅ rustup is already installed!${RESET}"
else
    echo -e "${YELLOW}❌ rustup not found...${RESET}"
    echo -e "${CYAN}🚀 Installing rustup, please wait...${RESET}"

    # Download rustup-init
    curl -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh
    chmod +x /tmp/rustup-init.sh

    # Run installer with spinner
    /tmp/rustup-init.sh -y &
    spinner $! "Installing rustup"

    echo -e "${GREEN}✅ rustup installed!${RESET}"
fi

# Build and install rust-cli
BIN_NAME="rust-cli"
INSTALL_DIR="$HOME/.cargo/bin"

echo -e "${CYAN}🔨 Building rust-cli...${RESET}"
cargo build --release &
spinner $! "Building rust-cli"

echo -e "${CYAN}📦 Installing rust-cli to $INSTALL_DIR...${RESET}"
mkdir -p "$INSTALL_DIR"
cp "target/release/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"

echo -e "${GREEN}🎉 Installed successfully!${RESET}"
echo -e "${MAGENTA}Run: $BIN_NAME --help${RESET}"
