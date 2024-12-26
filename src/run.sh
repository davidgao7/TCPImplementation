#!/bin/bash
set -e # ensures the script exits if any command fails
# ensure run.sh uses unix-style line endings (\n) instead of windows-style(\r\n)

# Create and configure the TUN interface
ip tuntap add dev tun0 mode tun || true
ip link set dev tun0 up || true
# add tun0 ip addr
ip addr add 192.168.2.1/24 dev tun0 || true
ip route add 192.168.2.0/24 dev tun0 || true

# 1. build the rust project
cargo b --release

# 2. add executable permission (no need as I'm root)
# setcap cap_net_admin=eip ../target/release/tcp_implementation

# 3. run rust executable in background
target/release/tcp_implementation
pid=$! # create a thread to wait response

# 5. check ip addr
# ip addr
# wait process to finish
trap 'kill $pid' INT TERM # terminate program normally when use `C-c`
wait $pid
