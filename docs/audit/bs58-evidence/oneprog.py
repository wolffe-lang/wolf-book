# bs58: write every one-machine (lupin-run / wolf-run) program and every
# pending row's program to OUT/<slug>/<slug>.lu, assembling part(name) fences
# in page order (a `part(name, cont)` fence continues the earlier ones).
import os,re,sys,shutil
root,out=sys.argv[1],sys.argv[2]
want_ids=set(sys.argv[3].split(',')) if len(sys.argv)>3 else set()
os.makedirs(out,exist_ok=True)
rows=[]
for d,_,fs in os.walk(root+'/book'):
    for f in sorted(fs):
        if not f.endswith('.md'): continue
        p=os.path.join(d,f)
        if 'back/' in p: continue
        L=open(p).read().split('\n'); parts={}; i=0; sidx=0
        while i<len(L):
            m=re.match(r'^```wolf(,(.*))?$',L[i])
            if m:
                j=i+1
                while j<len(L) and not L[j].startswith('```'): j+=1
                body='\n'.join(L[i+1:j]); dire=m.group(2) or ''
                pm=re.match(r'part\((\w+)(, *cont)?\)',dire)
                if pm:
                    parts[pm.group(1)]=(parts.get(pm.group(1),'')+'\n'+body) if pm.group(2) else body
                    prog=parts[pm.group(1)]
                else: prog=body
                rest=re.sub(r'^part\([^)]*\),?','',dire)
                rows.append((os.path.relpath(p,root)+':'+str(i+1),rest,prog,pm.group(1) if pm else None))
                i=j
            i+=1
for d,_,fs in os.walk(root+'/principles/exercises'):
    for f in sorted(fs):
        if f.endswith('.lu'):
            p=os.path.join(d,f); s=open(p).read(); m=re.search(r'//! check: (.*)',s)
            rows.append((os.path.relpath(p,root),m.group(1) if m else '',s,None))
n=0
for loc,dire,prog,part in rows:
    if not (('lupin-run' in dire) or ('wolf-run' in dire) or any(w in loc for w in want_ids)): continue
    slug=re.sub(r'[^A-Za-z0-9]+','_',loc.replace('principles/exercises/','').replace('book/',''))
    od=os.path.join(out,slug); os.makedirs(od,exist_ok=True)
    open(os.path.join(od,slug+'.lu'),'w').write(prog+'\n')
    open(os.path.join(od,'DIRECTIVE'),'w').write(loc+' | '+dire+'\n'); n+=1
print(n,'programs')
