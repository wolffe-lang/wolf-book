# bs54 static detectors for 0.2.18's two refusals, over a pickle of samples
# or over a directory of .lu files. D464: a `mut` parameter moved out in the
# callee. D460: an element moved (move/take of an index, or a plain binding
# of an index read) anywhere in the program.
import pickle,re,sys,os
def load(arg):
    if arg.endswith('.pkl'): return pickle.load(open(arg,'rb'))
    return [(f,'?',open(os.path.join(arg,f)).read()) for f in sorted(os.listdir(arg)) if f.endswith('.lu')]
def d464(src):
    L=src.split('\n'); out=[]
    for i,l in enumerate(L):
        m=re.search(r'\bfn\s+\w+(\[[^\]]*\])?\s*\((.*?)\)\s*(->|\{|$)',l)
        if not m: continue
        ps=re.findall(r'\bmut\s+(self|\w+)\s*[:.]',m.group(2))
        if not ps: continue
        ind=len(l)-len(l.lstrip()); body=[]
        if l.rstrip().endswith('}') and '{' in l: body=[(i,l)]
        else:
            j=i+1
            while j<len(L):
                body.append((j,L[j]))
                if L[j].startswith(' '*ind+'}') and len(L[j])-len(L[j].lstrip())==ind: break
                j+=1
        for p in ps:
            pp=re.escape(p)
            rx=re.compile(r'(take|move)\s+\(?'+pp+r'\b|=\s*'+pp+r'\s*(\.[\w.]+)?\s*(\[.*\])?\s*($|else|//)|'+pp+r'\.(pop|remove)\b')
            out+=[(n+1,x.strip()) for n,x in body if rx.search(x)]
    return out
def d460(src):
    L=src.split('\n')
    rx=re.compile(r'\b(move|take)\s+\(?[A-Za-z_][\w.]*\[|\b(let|var)\s+\w+\s*=\s*[A-Za-z_][\w.]*\[[^\]]+\]\s*($|//|else)')
    return [(i+1,l.strip()) for i,l in enumerate(L) if rx.search(l) and not l.strip().startswith('//')]
S=load(sys.argv[1]); h4=h0=0
for loc,d,src in S:
    a=d464(src); b=d460(src)
    if a: h4+=1; print('D464',loc,d,a)
    if b: h0+=1; print('D460',loc,d,b)
print('programs',len(S),'D464 hits',h4,'D460 hits',h0)
