# bs63: compare two sweep files program by program, machine by machine.
# Prints every program whose lupin exit/stdout, native or checked verdict/stdout/codes,
# or release exit/stdout moved. Args: a.jsonl b.jsonl
import json,sys
def load(p): return {r['id']:r for r in map(json.loads,open(p))}
A,B=load(sys.argv[1]),load(sys.argv[2])
moved=0
def summ(r):
    l=r['lupin']; n=r['native']; c=r['checked']; x=r['release']
    return {'lupin':(l['exit'],l['out']), 'native':(n.get('v'),n.get('out'),tuple(n.get('diag') or []),n.get('unsup')), 'checked':(c.get('v'),c.get('out'),tuple(c.get('diag') or []),c.get('unsup')), 'release':(x['exit'],x['out'])}
cnt={'lupin':0,'native':0,'checked':0,'release':0}
for k in sorted(set(A)|set(B)):
    if k not in A or k not in B: print('MISSING',k); continue
    a,b=summ(A[k]),summ(B[k])
    for m in a:
        if a[m]!=b[m]:
            cnt[m]+=1; moved+=1
            print(f'{k} [{m}]\n   was {str(a[m])[:400]}\n   now {str(b[m])[:400]}')
# release coverage
rel=sum(1 for r in B.values() if r['release']['exit']!='-')
print('programs',len(B),'release-built',rel,'moves',moved,cnt)
