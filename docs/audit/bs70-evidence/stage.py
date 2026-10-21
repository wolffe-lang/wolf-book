# bs70: build ch34.md in stages from the base (HEAD) and the full edit, by the diff's hunks, so each
# section lands in its own commit. Args: stage number (1..5); writes book/ch34.md.
import os,difflib,sys
H=os.path.expanduser('~/lanes/bs70/')
full=open(H+'ch34-full.md').read().split('\n'); base=open(H+'ch34-base.md').read().split('\n')
ops=[o for o in difflib.SequenceMatcher(None,base,full,autojunk=False).get_opcodes() if o[0]!='equal']
# split the big insert (the two sections) at "## 34.8"
hunks=[]
for o in ops:
    tag,i1,i2,j1,j2=o
    seg=full[j1:j2]
    if tag=='insert' and any(l.startswith('## 34.8') for l in seg):
        k=next(n for n,l in enumerate(seg) if l.startswith('## 34.8'))
        hunks.append(('s347',i1,i2,j1,j1+k)); hunks.append(('s348',i2,i2,j1+k,j2))
    else:
        hunks.append((tag,i1,i2,j1,j2))
def group(n,h):
    if n<2: return 1
    if n<11: return 2
    if h[0]=='s347': return 3
    if h[0]=='s348': return 4
    return 5
stage=int(sys.argv[1])
out=[]; pos=0
for n,h in enumerate(hunks):
    _,i1,i2,j1,j2=h
    out+=base[pos:i1]
    if group(n,h)<=stage: out+=full[j1:j2]
    else: out+=base[i1:i2]
    pos=i2
out+=base[pos:]
open(os.path.expanduser('~/lanes/bs70/wolf-book/book/ch34.md'),'w').write('\n'.join(out))
print('stage',stage,'hunks',[group(n,h) for n,h in enumerate(hunks)])
