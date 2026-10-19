#!/usr/bin/env bash
# bs69 on hasu (NixOS, no container runtime): run a script inside its OWN PID NAMESPACE, never on the
# host's process table (RULES.md §5): an unprivileged user namespace with the lane's own uid mapped
# (`unshare --user --map-current-user`), a new pid namespace with its own /proc (`--pid --fork
# --mount-proc`), so a signal from inside reaches only what the namespace started. RLIMIT_CORE 0 (hasu
# pipes cores to systemd-coredump with `ulimit -c` unlimited by default; it processes none at 0). The
# toolchain's linker: an ld.lld ALREADY in the nix store, put first on PATH (no package installed; as
# bs65 did), without which every `wolf build` console block carries wolf's linker note.
# Prints the namespace's view, SigBlk/SigIgn and the core limit first. Args: script [args...]
LLD=/nix/store/2c5r0kpn5yb8qsvfw77j8h63pklij8rq-lld-21.1.8/bin
exec unshare --user --map-current-user --pid --fork --mount-proc env PATH=$LLD:$PATH SUBJECT_HEAD=${SUBJECT_HEAD:-} bash -c 'ulimit -c 0; echo "namespace: pid $$ procs $(ls -d /proc/[0-9]* | wc -l) uid $(id -u) core $(ulimit -c)/$(ulimit -Hc) ld.lld $(command -v ld.lld)"; grep -E "^Sig(Blk|Ign)" /proc/self/status; exec bash "$@"' ns "$@"
