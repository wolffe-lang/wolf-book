#!/bin/bash
# bs68: run a script INSIDE a container, never on the host: podman, rootless, --userns=keep-id (the
# lane's own uid, so ids read as a user's), its own pid namespace (a signal can reach only what the
# container started), --network none, the host's /usr read-only (cargo, rustc, ld.lld at the host's
# versions), ~/.cargo and ~/.rustup for the toolchain the host already has, and the lane directory.
# Prints the container's SigBlk/SigIgn first: no signal row is called red without them.
# Args: script [args...] (a path under ~/lanes/bs68).
L=$HOME/lanes/bs68
exec podman run --rm --network none --userns=keep-id --init=false \
  -v /usr:/usr:ro -v $HOME/.rustup:$HOME/.rustup -v $HOME/.cargo:$HOME/.cargo -v $L:$L \
  -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin:/bin:$HOME/.cargo/bin -e TERM=dumb -e CARGO_NET_OFFLINE=true \
  -e SUBJECT_HEAD -e PAIR -w $L docker.io/library/archlinux:base-devel \
  /usr/bin/bash -c 'echo "container: $(hostname) pid $$ procs $(ls -d /proc/[0-9]* | wc -l) uid $(id -u)"; grep -E "^Sig(Blk|Ign)" /proc/self/status; exec bash "$@"' ctr "$@"
