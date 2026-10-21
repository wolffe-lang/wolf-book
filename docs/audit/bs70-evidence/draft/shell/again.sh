#!/bin/bash
# bs70: wolf-book#87's row, the way a shell reads it: §34.6's again.lu built and run with its marker
# argument, so it ends by terminate() (os_exit_signal), and `$?` read by bash. Run INSIDE the container,
# in a scratch directory, against the pair named by $W. RLIMIT_CORE 0 (terminate() writes no core anyway).
set -u
ulimit -c 0
cp "$1" ./again.lu
echo '$ wolf build again.lu && ./again again'
"$W" build again.lu && ./again again
echo '$ echo $?'
echo $?
