#!/bin/bash
# bs64: the s217 transcripts the runner does not replay, from real runs on the new pair.
#  (1) exercise 24-6's walkthrough: regex declares nothing, `wolf update`, then regex's manifest
#      declares net, then `wolf audit --ci`.
#  (2) the under-report check s217 adds: a dependency with caps=[] that calls fs_read_text with no
#      import — the build's E1504 and the audit's UNDECLARED line (and the same on the control pair).
set -u
L=$HOME/lanes/bs64; B=$L/wolf-book/samples/pkg
for P in new c25; do
W=$(ls $L/$P/wolf-*/wolf); T=$(mktemp -d)
echo "## pair=$P $($W --version | head -1)"
cp -r $B/acquired $T/a; cd $T/a
sed -i 's/capabilities: \[net\],/capabilities: [],/' regex/wolf.pkg
echo "--- (1) 24-6"
echo '$ wolf update --dir app'; $W update --dir app 2>&1; echo "[exit $?]"
sed -i 's/capabilities: \[\],/capabilities: [net],/' regex/wolf.pkg
echo '$ wolf audit --ci --dir app'; $W audit --ci --dir app 2>&1; rc=$?; echo '$ echo $?'; echo $rc
cd $L; cp -r $B/shelf $T/s; cd $T/s
printf '\npub fn peek() -> int {\n    (fs_read_text("rows.txt") else "").len\n}\n' >> rows/rows.lu
echo "--- (2) a dependency that calls fs_read_text and declares nothing"
echo '$ wolf audit --dir app'; $W audit --dir app 2>&1; echo "[exit $?]"
echo '$ wolf audit --ci --dir app'; $W audit --ci --dir app 2>&1; echo "[exit $?]"
echo '$ (cd app && wolf build main.lu)'; (cd app && $W build main.lu 2>&1 | head -30; echo "[exit ${PIPESTATUS[0]}]")
cd /; rm -rf $T
done
