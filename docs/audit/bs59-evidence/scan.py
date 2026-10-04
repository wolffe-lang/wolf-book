# bs59 static scans over every program text the samples runner executes
# (book fences outside back/solutions.md, every corpus .lu), aimed at what
# wolf 0.2.22 and lupin 0.1.45 changed. Run: python3 scan.py <wolf-book root>
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
IMPL={'trusted','consttime','allow','index','budget','repr','cfg'}
INT=r'(?:int|uint|i8|i16|i32|i64|u8|u16|u32|u64)'
hits={k:[] for k in ['attr_E0817','abi_E0818','ptr_sig_private','int_cast','for_view_region','view_in_region','trim_arg','fs_open','fs_handle_print','return_call_unit','nested_fn_moded','export_fn','extern_c','cfg','copy_in_region']}
for loc,d,src in S:
    L=src.split('\n')
    inreg=0
    for i,l in enumerate(L):
        s=l.split('//')[0]
        for a in re.findall(r'#\[\s*(\w+)',s):
            if a not in IMPL: hits['attr_E0817'].append((loc,d,i+1,l.strip()))
        if re.search(r'extern\s+"(?!c")',s): hits['abi_E0818'].append((loc,d,i+1,l.strip()))
        if re.match(r'\s*fn\s+\w+.*\*\w',s) and not re.match(r'\s*pub',s): hits['ptr_sig_private'].append((loc,d,i+1,l.strip()))
        for m in re.finditer(r'\bas\s+('+INT+r')\b',s):
            if m.group(1) not in ('int',): hits['int_cast'].append((loc,d,i+1,l.strip()))
        if re.search(r'\bfor\s+\w+\s+in\s+.*\.(words|lines|split)\(',s) and re.search(r'\bregion\b|\bin\s+\w+\s*\{',src): hits['for_view_region'].append((loc,d,i+1,l.strip()))
        if re.search(r'\.trim\w*\(\s*[^)\s]',s): hits['trim_arg'].append((loc,d,i+1,l.strip()))
        if 'fs_open' in s: hits['fs_open'].append((loc,d,i+1,l.strip()))
        if re.search(r'fs_open',src) and re.search(r'print\(.*\{(fd|h|f|file|handle)\}',s): hits['fs_handle_print'].append((loc,d,i+1,l.strip()))
        if re.search(r'\breturn\s+\w+\(.*\)\s*$',s): hits['return_call_unit'].append((loc,d,i+1,l.strip()))
        if re.match(r'\s{2,}fn\s+\w+\(.*\b(mut|take)\s+\w+:',s): hits['nested_fn_moded'].append((loc,d,i+1,l.strip()))
        if re.search(r'\bexport\s+fn\b',s): hits['export_fn'].append((loc,d,i+1,l.strip()))
        if re.search(r'extern\s+"c"',s): hits['extern_c'].append((loc,d,i+1,l.strip()))
        if 'cfg(' in s: hits['cfg'].append((loc,d,i+1,l.strip()))
        if re.search(r'\bcopy\s+\w+',s) and re.search(r'\bregion\b',src): hits['copy_in_region'].append((loc,d,i+1,l.strip()))
for k,v in hits.items():
    print('==',k,len(v))
    for x in v: print('  ',*x)
