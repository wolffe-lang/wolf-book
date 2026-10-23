#!/usr/bin/env bash
set -euo pipefail
export PATH=$HOME/lanes/hasu-env/bin:$HOME/lanes/hasu-env/profile/bin:$PATH
base=$HOME/lanes/bs72
exec flock "$HOME/lanes/hasu-A.lock" podman run --rm --memory 12g --network none --ulimit core=0 --pids-limit 512 \
 -v "$base:$base" -v /nix/store:/nix/store:ro -v "$HOME/lanes/hasu-env/profile:/tools:ro" \
 -v /run/current-system/sw:/host-system:ro -w "$base/wolf-book" \
 -e PATH=/tools/bin:/host-system/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin \
 -e WOLF="$base/release-wolf/wolf-0.2.35-x86_64-unknown-linux-gnu/wolf" \
 -e LUPIN="$base/release-lupin/lupin-0.1.56-x86_64-unknown-linux-gnu/lupin" \
 docker.io/library/ubuntu:24.04 taskset -c 0-5 "$@"
