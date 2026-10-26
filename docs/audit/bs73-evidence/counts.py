from pathlib import Path
from collections import Counter
import hashlib,json,re,subprocess
root=Path('/home/mfwolffe/lanes/bs73/wolf-book');control='591cd5d0779764b7bf043ebe712ec52e1e356aa5'
def before(path):return subprocess.check_output(['git','-C',str(root),'show',control+':'+str(path)])
lines=[]
p=Path('vendor/spec/anchors.json');a=json.loads(before(p))['anchors'];b=json.loads((root/p).read_text())['anchors']
lines+=['anchor keys '+str(len(a))+' -> '+str(len(b)), 'removed '+str(sorted(a.keys()-b.keys())), 'added '+str(sorted(b.keys()-a.keys())), 'retargeted '+str([k for k in a.keys()&b.keys() if a[k]!=b[k]])]
p=Path('vendor/spec/diagnostic-codes.txt');a=set(before(p).decode().splitlines());b=set((root/p).read_text().splitlines())
lines+=['diagnostic codes '+str(len(a))+' -> '+str(len(b)), 'removed '+str(sorted(a-b)), 'added '+str(sorted(b-a))]
p=Path('vendor/spec/grammar.ebnf');a=before(p);b=(root/p).read_bytes();assert a==b
lines+=['grammar unchanged sha256 '+hashlib.sha256(b).hexdigest()]
counts=Counter();ub=0
for p in sorted((root/'book').rglob('*.md'))+sorted((root/'principles/exercises').rglob('*.lu')):
 if p.name in ('SUMMARY.md','solutions.md') or p.name.startswith('_'):continue
 s=re.sub(r'<!--.*?-->','',p.read_text(),flags=re.S)
 for d in re.findall(r'^```wolf,([^\n]*)' if p.suffix=='.md' else r'^//! check: (.*)',s,re.M):
  m=re.search(r'(?<![\w-])(lupin-run|wolf-run|run|fail)\(',d)
  if m:counts[m[1]]+=1;ub+=m[1]=='lupin-run' and 'trap(ub)' in d
lines+=['active directives '+str(dict(sorted(counts.items()))), 'lupin UB '+str(ub)+'; eligible lupin-run '+str(counts['lupin-run']-ub), 'README read, current; no command, layout or chapter-count change.']
out=root/'docs/audit/bs73-evidence/counts.txt';out.write_text('\n'.join(lines)+'\n');print(out.read_text())
