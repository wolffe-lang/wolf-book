import concurrent.futures, json, os, pathlib, re, subprocess, time, signal
root=pathlib.Path('/home/mfwolffe/lanes/bs73/wolf-book')
files=sorted(p for p in (root/'principles/exercises').rglob('*.lu') if re.search(r'^//! check:',p.read_text(),re.M))
logs=root/'docs/audit/bs73-evidence/census';logs.mkdir(parents=True,exist_ok=True)
def run(p):
 text=p.read_text(); claim=re.search(r'^//! check: (.+)',text,re.M)[1]; rel=str(p.relative_to(root)); start=time.monotonic()
 cmd=[os.environ['WOLF'],'run','./'+rel]
 try:
  proc=subprocess.Popen(cmd,cwd=root,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,start_new_session=True)
  try:
   out,err=proc.communicate(timeout=8);rc=proc.returncode
  except subprocess.TimeoutExpired:
   # Record this command's descendants, then end only those exact pids.
   records=[x.split() for x in subprocess.check_output(['ps','-eo','pid,ppid'],text=True).splitlines()[1:]]
   owned={proc.pid}
   for _ in range(12): owned.update(int(p) for p,parent in records if int(parent) in owned)
   for pid in sorted(owned,reverse=True):
    try: os.kill(pid,signal.SIGKILL)
    except ProcessLookupError: pass
   out,err=proc.communicate(timeout=3);rc=None
 except subprocess.TimeoutExpired as e:
  rc=None;out=(e.stdout or b'').decode() if isinstance(e.stdout,bytes) else e.stdout or '';err=(e.stderr or b'').decode() if isinstance(e.stderr,bytes) else e.stderr or ''
 status='timeout' if rc is None else 'rejected' if rc==2 else 'refused' if rc==4 else 'ran'
 row=dict(file=rel,claim=claim,status=status,exit=rc,seconds=round(time.monotonic()-start,3),stdout=out,stderr=err)
 (logs/(p.stem+'.json')).write_text(json.dumps(row,indent=2)+'\n');return row
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: rows=list(pool.map(run,files))
(root/'docs/audit/bs73-evidence/census.json').write_text(json.dumps(rows,indent=2)+'\n')
from collections import Counter
print('RAW NATIVE',len(rows),dict(Counter(r['status'] for r in rows)))
for r in rows:
 if r['status']!='ran':print(r['file'],r['claim'],r['status'],r['exit'])
