#!/bin/bash
# bs70 (from bs69, bs68): run a script INSIDE a container, never on the host: podman, rootless, --userns=keep-id
# (the lane's own uid, so ids read as a user's), its own pid namespace (a signal can reach only what the
# container started), --network none, the host's /usr read-only (cargo, rustc, ld.lld at the host's
# versions), ~/.cargo and ~/.rustup for the toolchain the host already has, and the lane directory.
# bs70: RLIMIT_CORE 0 soft and hard (--ulimit core=0:0, and ulimit -c 0 again inside): kasumi pipes cores
# to systemd-coredump, which processes a dump only when the crashing process's RLIMIT_CORE allows one, so
# no program this lane runs can leave a core on the host.
# Prints the container's SigBlk/SigIgn and core limit first: no signal row is called red without them.
# Args: script [args...] (a path under ~/lanes/bs70).
L=$HOME/lanes/bs70
exec podman run --rm --network none --userns=keep-id --init=false --ulimit core=0:0 \
  -v /usr:/usr:ro -v $HOME/.rustup:$HOME/.rustup -v $HOME/.cargo:$HOME/.cargo -v $L:$L \
  -e HOME=$HOME -e PATH=/usr/local/bin:/usr/bin:/bin:$HOME/.cargo/bin -e TERM=dumb -e CARGO_NET_OFFLINE=true \
  -e SUBJECT_HEAD -e PAIR -w $L docker.io/library/archlinux:base-devel \
  /usr/bin/bash -c 'ulimit -c 0; echo "container: $(hostname) pid $$ procs $(ls -d /proc/[0-9]* | wc -l) uid $(id -u) core $(ulimit -c)/$(ulimit -Hc)"; grep -E "^Sig(Blk|Ign)" /proc/self/status; exec bash "$@"' ctr "$@"
