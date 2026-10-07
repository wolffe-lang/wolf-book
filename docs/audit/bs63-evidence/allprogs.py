# bs63: write EVERY program the samples runner executes (book wolf fences outside
# back/, part(name) fences assembled in page order as bs60's oneprog.py does, and
# every corpus .lu) to OUT/<slug>/<slug>.lu with its directive in DIRECTIVE, so each
# runs alone in its own directory. Run: python3 allprogs.py <wolf-book root> <out>
import os,re,sys
root,out=sys.argv[1],sys.argv[2]
os.makedirs(out,exist_ok=True)
rows=[]
for d,_,fs in os.walk(root+'/book'):
    for f in sorted(fs):
        if not f.endswith('.md'): continue
        p=os.path.join(d,f)
        if '/back/' in p: continue
        L=open(p).read().split('\n'); parts={}; i=0
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
                if 'ignore' not in rest and 'fragment' not in rest:
                    rows.append((os.path.relpath(p,root)+':'+str(i+1),rest,prog))
                i=j
            i+=1
for d,_,fs in os.walk(root+'/principles/exercises'):
    for f in sorted(fs):
        if f.endswith('.lu'):
            p=os.path.join(d,f); s=open(p).read(); m=re.search(r'//! check: (.*)',s)
            rows.append((os.path.relpath(p,root),m.group(1) if m else '',s))
for loc,dire,prog in rows:
    slug=re.sub(r'[^A-Za-z0-9]+','_',loc.replace('principles/exercises/','').replace('book/',''))
    od=os.path.join(out,slug); os.makedirs(od,exist_ok=True)
    open(os.path.join(od,slug+'.lu'),'w').write(prog if prog.endswith('\n') else prog+'\n')
    open(os.path.join(od,'DIRECTIVE'),'w').write(loc+' | '+dire+'\n')
print(len(rows),'programs')
