# bs70: the head sweep against the unedited tree's, for the programs whose slug (a page line) moved:
# matched by program TEXT instead of by slug. Args: a.jsonl a-progs-dir b.jsonl b-progs-dir
import json,sys,os
def load(p,d):
    out={}
    for r in map(json.loads,open(p)):
        f=os.path.join(d,r['id'],r['id']+'.lu')
        out.setdefault(open(f).read(),[]).append(r)
    return out
A=load(sys.argv[1],sys.argv[2]); B=load(sys.argv[3],sys.argv[4])
def summ(r):
    l=r['lupin']; n=r['native']; c=r['checked']; x=r['release']
    return (l['exit'],l['out'],n.get('v'),n.get('out'),c.get('v'),c.get('out'),x['exit'],x['out'])
same=diff=0; new=[]
for t,rs in B.items():
    if t not in A: new.append(rs[0]['id']); continue
    if summ(A[t][0])==summ(rs[0]): same+=1
    else: diff+=1; print('MOVED',rs[0]['id'])
gone=[rs[0]['id'] for t,rs in A.items() if t not in B]
print(f'by text: {same} identical records, {diff} moved; only at the head: {sorted(new)}; only before: {sorted(gone)}')
