import os,re,sys
import sys; root=sys.argv[1] if len(sys.argv)>1 else '.'
out=[]
for d,_,fs in os.walk(root+'/book'):
    for f in sorted(fs):
        if not f.endswith('.md'): continue
        p=os.path.join(d,f)
        if 'back/solutions' in p: continue
        L=open(p).read().split('\n')
        i=0
        while i<len(L):
            m=re.match(r'^```wolf,?(.*)$',L[i])
            if m and not L[i].startswith('```wolf-repl'):
                j=i+1
                while j<len(L) and not L[j].startswith('```'): j+=1
                out.append((os.path.relpath(p,root)+':'+str(i+1),m.group(1),'\n'.join(L[i+1:j])))
                i=j
            i+=1
for d,_,fs in os.walk(root+'/principles/exercises'):
    for f in sorted(fs):
        if f.endswith('.lu'):
            p=os.path.join(d,f); s=open(p).read()
            m=re.search(r'//! check: (.*)',s)
            out.append((os.path.relpath(p,root),m.group(1) if m else '?',s))
import pickle; pickle.dump(out,open('samples.pkl','wb'))
print(len(out))
