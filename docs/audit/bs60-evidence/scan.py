# bs60 static scans over every program text the samples runner executes
# (book fences outside back/solutions.md, every corpus .lu), aimed at what
# wolf 0.2.23 and lupin 0.1.46 changed. Run: python3 scan.py <wolf-book root>
import os,re,sys
root=sys.argv[1] if len(sys.argv)>1 else '.'
S=[]
for d,_,fs in os.walk(root+'/book'):
    for f in sorted(fs):
        if not f.endswith('.md'): continue
        p=os.path.join(d,f)
        if 'back/solutions' in p: continue
        L=open(p).read().split('\n'); i=0
        while i<len(L):
            m=re.match(r'^```wolf,?(.*)$',L[i])
            if m and not L[i].startswith('```wolf-repl'):
                j=i+1
                while j<len(L) and not L[j].startswith('```'): j+=1
                S.append((os.path.relpath(p,root)+':'+str(i+1),m.group(1),'\n'.join(L[i+1:j]))); i=j
            i+=1
for d,_,fs in os.walk(root+'/principles/exercises'):
    for f in sorted(fs):
        if f.endswith('.lu'):
            p=os.path.join(d,f); s=open(p).read(); m=re.search(r'//! check: (.*)',s)
            S.append((os.path.relpath(p,root),m.group(1) if m else '?',s))
print('programs',len(S))
K=['module_state','prefix_deref','int_ptr_cast','prov_method','layout_query','repr_attr','section_attr','extern_let','volatile','attr_on_lupin','errdefer','annot_row_binding','elseless_if_fallible_tail','copy_str','cfg','e0708_word']
hits={k:[] for k in K}
IMPL_LUPIN_OK={'trusted','consttime','allow','index','budget','repr','cfg'}
for loc,d,src in S:
    L=src.split('\n')
    for i,l in enumerate(L):
        s=l.split('//')[0]
        if re.match(r'(pub(\(pkg\))?\s+)?(let|var|const)\s+\w',s): hits['module_state'].append((loc,d,i+1,l.strip()))
        if re.search(r'(^|[=(\s,{])\*[a-z_]\w*',s) and not re.search(r':\s*\*\w',s) and not re.search(r'\w\s*\*\s*[a-z_]',s): hits['prefix_deref'].append((loc,d,i+1,l.strip()))
        if re.search(r'\bas\s+\*\w|\*\w+\s+as\s+(int|uint|u64|i64|u8|u16|u32|i32)\b',s) or re.search(r'\)\s*as\s+(int|uint|u64)\b',s) and '*' in s: hits['int_ptr_cast'].append((loc,d,i+1,l.strip()))
        if re.search(r'\.(addr|with_addr|expose|with_exposed|is_null)\(',s): hits['prov_method'].append((loc,d,i+1,l.strip()))
        if re.search(r'\b(size_of|align_of|offset_of)\(',s): hits['layout_query'].append((loc,d,i+1,l.strip()))
        if re.search(r'#\[\s*repr',s): hits['repr_attr'].append((loc,d,i+1,l.strip()))
        if re.search(r'#\[\s*(section|link_section)',s): hits['section_attr'].append((loc,d,i+1,l.strip()))
        if re.search(r'extern\s+"\w*"\s+let',s): hits['extern_let'].append((loc,d,i+1,l.strip()))
        if re.search(r'(read|write)_volatile',s): hits['volatile'].append((loc,d,i+1,l.strip()))
        for a in re.findall(r'#\[\s*(\w+)',s):
            hits['attr_on_lupin'].append((loc,d,i+1,l.strip()))
        if re.search(r'\berrdefer\b',s): hits['errdefer'].append((loc,d,i+1,l.strip()))
        if re.search(r'\blet\s+\w+\s*:\s*[^=]*!\s*\{',s): hits['annot_row_binding'].append((loc,d,i+1,l.strip()))
        if re.search(r'\bcopy\s+\w',s): hits['copy_str'].append((loc,d,i+1,l.strip()))
        if 'cfg(' in s: hits['cfg'].append((loc,d,i+1,l.strip()))
    # else-less if as the tail of a fallible fn: track fn bodies by brace depth
    i=0
    while i<len(L):
        m=re.match(r'(\s*)(pub\s+)?fn\s+\w+.*\)\s*->\s*([^{]*)\{\s*$',L[i])
        if m and '!' in m.group(3):
            ind=m.group(1); depth=0; j=i; body=[]
            while j<len(L):
                depth+=L[j].count('{')-L[j].count('}'); body.append((j,L[j]))
                if depth==0 and j>i: break
                j+=1
            # find the last top-level statement of the body (indent = ind+4)
            inner=[(k,x) for k,x in body[1:-1] if x.strip() and not x.strip().startswith('//')]
            tops=[(k,x) for k,x in inner if re.match(re.escape(ind)+r'    \S',x)]
            if tops:
                k,x=tops[-1]
                # the tail statement starts at the last top-level line not starting with '}' or 'else'
                st=[t for t in tops if not t[1].strip().startswith('}')]
                if st:
                    k0,x0=st[-1]
                    if x0.strip().startswith('if ') or x0.strip().startswith('if('):
                        tail='\n'.join(xx for kk,xx in body if kk>=k0)
                        if 'else' not in tail: hits['elseless_if_fallible_tail'].append((loc,d,k0+1,L[i].strip()+' … '+x0.strip()))
            i=j
        i+=1
for k,v in hits.items():
    print('==',k,len(v))
    for x in v: print('  ',*x)
