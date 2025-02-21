#!/bin/bash
set -e # Exit immediately if a command fails

# Clean up existing tun0 interface if it exists
if ip link show tun0 >/dev/null 2>&1; then
    echo "Cleaning up existing tun0 interface..."
    sudo ip link set tun0 down
    sudo ip tuntap del dev tun0 mode tun
else
    echo "tun0 does not exist, skipping cleanup."
fi

# Create tun0 interface
echo "Creating tun0 interface..."
sudo ip tuntap add dev tun0 mode tun
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set tun0 up

# Build the Rust project only if the binary doesn't exist
echo "Building the Rust project..."
cargo build --release

ext=$?
if [[ $ext -ne 0 ]]; then
    exit $ext
fi

# Run the Rust program
echo "Running the program..."
target/release/tcp_implementation &
pid=$!

# Ensure cleanup on exit
trap 'kill $pid; sudo ip link set tun0 down; sudo ip tuntap del dev tun0 mode tun' EXIT

# Wait for the process to complete
wait $pid
