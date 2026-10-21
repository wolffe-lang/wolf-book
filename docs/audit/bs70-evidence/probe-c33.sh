#!/bin/bash
# bs70, INSIDE the container (ctr.sh): the four new pages' drafts on the CONTROL pair (wolf 0.2.33 /
# lupin 0.1.54), to show the pages' programs reach names 0.2.33 does not have.
set -u
L=$HOME/lanes/bs70; export TMPDIR=$L/tmp
PAIR=c33 bash $L/ev/probes.sh $L/wolf-book/docs/audit/bs70-evidence/draft > $L/logs/probes-draft-c33.txt 2>&1
echo DONE > $L/logs/probe-c33.done
