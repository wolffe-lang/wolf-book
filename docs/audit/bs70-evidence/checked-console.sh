#!/bin/bash
# bs70, INSIDE the container (ctr.sh): the checked machine's reader half for a draft, the way the console
# replay runs it (`wolf conform-run --checked <file>`, stdout and stderr kept apart). Args: draft file.
set -u
L=$HOME/lanes/bs70; export TMPDIR=$L/tmp; W=$(ls $L/new/wolf-*/wolf)
T=$(mktemp -d -p $TMPDIR); cp "$1" $T/; cd $T; f=$(basename "$1")
echo "\$ wolf conform-run --checked $f"; $W conform-run --checked $f > o.txt 2> e.txt < /dev/null; echo "[exit $?]"
echo "--- stdout"; cat o.txt; echo "--- stderr"; cat e.txt
cd $L; rm -rf $T
