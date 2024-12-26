#!/bin/bash
set -e # ensures the script exits if any command fails
# ensure run.sh uses unix-style line endings (\n) instead of windows-style(\r\n)

# 1. build the rust project
cargo b --release

# 2. add executable permission (no need as I'm root)
# setcap cap_net_admin=eip ../target/release/tcp_implementation

# 3. run rust executable in background
target/release/tcp_implementation
pid=$! # create a thread to wait response
# 4. add tun0 ip addr
ip addr add 192.168.0.1/24 dev tun0
ip link set up dev tun0

# 5. check ip addr
# ip addr
# wait process to finish
trap 'kill $pid' INT TERM # terminate program normally when use `C-c`
wait $pid
