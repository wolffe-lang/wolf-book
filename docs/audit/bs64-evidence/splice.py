# bs64 (from bs63): regenerate drifted transcripts FROM THE RUN, never by hand.
#  - console blocks: the samples log's `FAIL <md>:<line>: console block drifted` entries carry
#    the block's `expected:` and the run's `actual:`; the block opening at <line> must equal
#    `expected` exactly, and its body becomes `actual`.
#  - diagnostic,from(<id>) blocks: the body becomes the reviewed snapshot
#    snapshots/diagnostics/<id with / -> __>.txt, which `samples --bless` wrote from the run.
# Run: python3 splice.py <wolf-book root> <samples log> [diag ids...]
import re,sys,os
root,log=sys.argv[1],sys.argv[2]; ids=sys.argv[3:]
L=open(log).read().split('\n'); i=0; done=0
def body(lines):
    out=[]
    for l in lines:
        m=re.match(r'^       \| ?(.*)$',l)
        if not m: break
        out.append(m.group(1))
    return out
# bs64: collect every drift first, then apply per file from the LAST block up, so a block that
# grows or shrinks never moves the line number the log gave for a block below it (bs63's splice
# applied them in log order and asserted on the second block of a file whose first block grew).
edits=[]
while i<len(L):
    m=re.match(r'^samples: FAIL (\S+?):(\d+): console block drifted from the real run$',L[i])
    if m:
        path,line=m.group(1),int(m.group(2))
        assert L[i+1].strip()=='expected:', L[i+1]
        exp=body(L[i+2:]); j=i+2+len(exp)
        assert L[j].strip()=='actual:', L[j]
        act=body(L[j+1:])
        rel=os.path.relpath(path,root) if path.startswith('/') else path
        edits.append((rel,line,exp,act))
        i=j+1+len(act); continue
    i+=1
for rel,line,exp,act in sorted(edits,key=lambda e:(e[0],-e[1])):
    f=os.path.join(root,rel); T=open(f).read().split('\n')
    k=line-1; assert T[k].startswith('```console'), (rel,line,T[k])
    e=k+1
    while not T[e].startswith('```'): e+=1
    assert T[k+1:e]==exp, (rel,line,'block is not the log\'s expected text')
    T[k+1:e]=act; open(f,'w').write('\n'.join(T)); done+=1
    print(f'console {rel}:{line}: {len(exp)} -> {len(act)} lines')
for id_ in ids:
    snap=open(os.path.join(root,'snapshots/diagnostics',id_.replace('/','__')+'.txt')).read().rstrip('\n').split('\n')
    hit=0
    for d,_,fs in os.walk(os.path.join(root,'book')):
        for fn in fs:
            if not fn.endswith('.md'): continue
            f=os.path.join(d,fn); T=open(f).read().split('\n'); changed=False
            for k,l in enumerate(T):
                if l.startswith('```diagnostic') and f'from({id_})' in l:
                    e=k+1
                    while not T[e].startswith('```'): e+=1
                    T[k+1:e]=snap; changed=True; hit+=1
                    print(f'diagnostic,from({id_}) {os.path.relpath(f,root)}:{k+1}: {e-k-1} -> {len(snap)} lines'); break
            if changed: open(f,'w').write('\n'.join(T))
    assert hit==1,(id_,hit)
    done+=1
print(done,'blocks regenerated')
