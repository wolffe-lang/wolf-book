# bs65: the subject sweep against the control, read against the prediction. Args: c26a.jsonl newa.jsonl
# predicted-checked.txt progs-dir. Prints every native/lupin move, and every checked move classed:
# PREDICTED-MATCH (one of the 83, lupin's verdict class and, on exit(0), lupin's bytes),
# PREDICTED-OTHER (one of the 83, another answer), UNPREDICTED (outside the 83).
import json,sys,collections
A={json.loads(l)['id']:json.loads(l) for l in open(sys.argv[1])}
B={json.loads(l)['id']:json.loads(l) for l in open(sys.argv[2])}
pred={l.split(' | ')[0] for l in open(sys.argv[3]) if not l.startswith('#')}
progs=sys.argv[4]
def cls(e):
    return 'exit(0)' if e==0 else ('trap' if e==3 else f'exit({e})')
def key(m):
    return (m.get('v'),m.get('out'),tuple(m.get('diag') or []),m.get('unsup'))
tally=collections.Counter()
for k in sorted(B):
    a,b=A[k],B[k]
    d=open(f'{progs}/{k}/DIRECTIVE').read().strip()
    for m in ('native','lupin'):
        x=(a[m]['exit'],a[m]['out']) if m=='lupin' else key(a[m]); y=(b[m]['exit'],b[m]['out']) if m=='lupin' else key(b[m])
        if x!=y: print(f'{m.upper()} MOVE {k} [{d}]\n   was {x}\n   now {y}')
    ca,cb=key(a['checked']),key(b['checked'])
    if ca==cb: continue
    l=b['lupin']; v=cb[0] or ''
    lv=cls(l['exit'])
    same_class = (v==lv) or (lv=='trap' and v.startswith('trap(')) or (lv!='exit(0)' and v.startswith('ub('))
    same_bytes = (v!='exit(0)') or (cb[1]==l['out'])
    if k in pred:
        c='PREDICTED-MATCH' if same_class and same_bytes else 'PREDICTED-OTHER'
    else: c='UNPREDICTED'
    tally[c]+=1
    if c!='PREDICTED-MATCH' or k in ('ch16_ex16_7_lu','ch16_ex16_8_lu'):
        print(f'{c} {k} [{d}]\n   checked was {ca}\n   checked now {cb}\n   lupin exit {l["exit"]} out {l["out"][:200]!r} err {l["err"][:200]!r}')
missing=[k for k in pred if key(A[k]['checked'])==key(B[k]['checked'])]
print('predicted rows that did NOT move:',missing)
print(dict(tally))
