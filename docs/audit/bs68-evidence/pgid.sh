#!/bin/bash
# bs68, INSIDE the container (ctr.sh): which process group does a program run in, by the route
# that runs it? `wolf run` directly, under timeout, and the way the samples runner spawns it
# (its own process group, stdin /dev/null, stdout and stderr pipes: python's setpgrp stands in for
# Rust's process_group(0)); and the checked machine.
L=$HOME/lanes/bs68; export TMPDIR=$L/tmp; W=$(ls $L/new/wolf-*/wolf)
T=$(mktemp -d -p $TMPDIR); cp $L/ev/pgid/pg.lu $T/; cd $T
echo "shell: pid $$ pgid $(ps -o pgid= -p $$)"
echo "\$ wolf run pg.lu"; $W run pg.lu < /dev/null
echo "\$ timeout 60 wolf run pg.lu"; timeout 60 $W run pg.lu < /dev/null
echo "\$ (runner's shape) wolf run pg.lu"; python3 -c 'import os,subprocess,sys; r=subprocess.run([sys.argv[1],"run","pg.lu"],stdin=subprocess.DEVNULL,capture_output=True,preexec_fn=os.setpgrp); print(r.stdout.decode(),r.stderr.decode(),"exit",r.returncode)' $W
echo "\$ (runner's shape) wolf build pg.lu && ./pg"; $W build pg.lu > /dev/null 2>&1; python3 -c 'import os,subprocess,sys; r=subprocess.run(["./pg"],stdin=subprocess.DEVNULL,capture_output=True,preexec_fn=os.setpgrp); print(r.stdout.decode(),r.stderr.decode(),"exit",r.returncode)'
echo "\$ wolf conform-run --json --checked pg.lu"; $W conform-run --json --checked pg.lu < /dev/null | grep -o '"stdout_inline":"[^"]*"'
cd $L; rm -rf $T
