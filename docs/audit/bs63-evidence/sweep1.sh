#!/bin/bash
# bs63: one program, one pair: lupin, conform-run --json (native), --checked, and on a
# native `pass` a release build and run. Prints one JSON line. Args: pairdir progdir
P=$1; D=$2; W=$(ls $P/wolf-*/wolf); U=$(ls $P/lupin-*/lupin)
cd "$D" || exit 1; f=$(basename "$D").lu
TMP=$(mktemp -d); trap 'rm -rf "$TMP"' EXIT
timeout 60 $U $f > $TMP/l.out 2> $TMP/l.err < /dev/null; le=$?
timeout 120 $W conform-run --json $f > $TMP/n.json 2> $TMP/n.err < /dev/null; ne=$?
timeout 120 $W conform-run --json --checked $f > $TMP/c.json 2> $TMP/c.err < /dev/null; ce=$?
re=-; rs=""
if grep -q '"verdict": *"pass"' $TMP/n.json && grep -q 'fn main' $f; then
  cp $f $TMP/r.lu; (cd $TMP && timeout 300 $W build --release r.lu > rb.out 2>&1 < /dev/null); rb=$?
  if [ $rb = 0 ] && [ -x $TMP/r ]; then (cd $TMP && timeout 60 ./r > r.out 2> r.err < /dev/null); re=$?; else re="build$rb"; fi
fi
python3 - "$D" $le $ne $ce "$re" $TMP <<'PY'
import json,sys,hashlib,os
d,le,ne,ce,re,t=sys.argv[1:7]
def rd(n):
    p=os.path.join(t,n)
    return open(p,'rb').read().decode('utf-8','replace') if os.path.exists(p) else ''
def rec(n):
    for l in rd(n).splitlines():
        if l.startswith('{'):
            try:
                r=json.loads(l); return {'v':r.get('verdict'),'out':r.get('stdout_inline'),'diag':[x.get('code') for x in r.get('diagnostics',[])],'unsup':r.get('x-unsupported-construct'),'trap':r.get('trap_kind') or r.get('x-trap-kind')}
            except Exception: pass
    return {'v':None,'raw':rd(n)[:200]}
print(json.dumps({'id':os.path.basename(d),'lupin':{'exit':int(le),'out':rd('l.out')[:2000],'err':rd('l.err')[:400]},'native':rec('n.json'),'checked':rec('c.json'),'release':{'exit':re,'out':rd('r.out')[:2000]}},sort_keys=True))
PY
