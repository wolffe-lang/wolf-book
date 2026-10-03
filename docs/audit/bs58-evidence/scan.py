# bs58 static scans over every program text (bs54's extract.py pickle).
import pickle,re,sys
S=pickle.load(open(sys.argv[1] if len(sys.argv)>1 else 'samples.pkl','rb'))
I32=2**31-1
def lit_val(t):
    t=t.replace('_','')
    try:
        if re.fullmatch(r'-?0x[0-9a-fA-F]+',t): return int(t,16)
        if re.fullmatch(r'-?\d+',t): return int(t)
    except: pass
    return None
hits={k:[] for k in ['defer_try','numlit','shorthand','match_any','assert_msg','errdefer_block','else_try_in_scrut','fnvalue_moded']}
for loc,d,src in S:
    L=src.split('\n')
    for i,l in enumerate(L):
        s=l.split('//')[0]
        if re.search(r'\b(defer|errdefer)\b',s) and '?' in s: hits['defer_try'].append((loc,i+1,l.strip()))
        m=re.match(r'\s*(let|var)\s+(\w+)\s*=\s*([-+*/% ()0-9_xa-fA-F]+?)\s*;?\s*$',s)
        if m and re.search(r'\d',m.group(3)) and not re.search(r'[A-Za-z_]\w*\s*\(',m.group(3)):
            expr=m.group(3).replace('_','')
            if re.fullmatch(r'[-+*/% ()0-9xa-fA-F]+',expr):
                try:
                    v=eval(expr.replace('/','//'))
                    if abs(v)>I32: hits['numlit'].append((loc,i+1,l.strip(),v))
                except Exception: pass
        if re.search(r'\b[A-Z]\w*\s*\{\s*\w+\s*(,\s*\w+\s*)*\}',s) and not re.search(r'\b(struct|enum|match|if|else|for|while|fn|impl)\b',s):
            hits['shorthand'].append((loc,i+1,l.strip()))
        if re.search(r'\bmatch\b',s): hits['match_any'].append((loc,i+1,l.strip()))
        if re.search(r'\bassert\s*\(.*,\s*[^")]',s): hits['assert_msg'].append((loc,i+1,l.strip()))
        if 'errdefer' in s: hits['errdefer_block'].append((loc,i+1,l.strip()))
        if re.search(r'\?.*\belse\b',s) and not re.search(r'\bif\b',s): hits['else_try_in_scrut'].append((loc,i+1,l.strip()))
for k,v in hits.items():
    if k=='match_any': print(k,len(v),'lines in',len({x[0] for x in v}),'programs'); continue
    print('==',k,len(v))
    for x in v: print('  ',*x)
