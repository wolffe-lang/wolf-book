#!/bin/bash
# bs70: wolf-book#87's row, the way shells read it: §34.6's again.lu built and run with its marker
# argument, so it ends by terminate() (os_exit_signal), and `$?` read right after. Run INSIDE the
# container, in a scratch directory, against the pair named by $W. RLIMIT_CORE 0 (terminate() writes no
# core anyway). Each shell runs the binary as its last foreground command and prints `$?`; what the shell
# itself writes about the death (a job notice) is captured on its own stream.
set -u
ulimit -c 0
cp "$1" ./again.lu
echo '$ wolf build again.lu && ./again again'
"$W" build again.lu && ./again again
rc=$?
echo '$ echo $?'
echo $rc
for sh in "bash -c" "sh -c" "dash -c" "zsh -c"; do
  set -- $sh; command -v $1 > /dev/null || { echo "## $sh: not installed here"; continue; }
  out=$($sh './again again; echo "status $?"' 2> err.txt); echo "## $sh: stdout [$out] stderr [$(cat err.txt)]"
done
