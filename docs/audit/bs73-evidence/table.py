from pathlib import Path
import json,collections,re
base=Path('/home/mfwolffe/lanes/bs73');root=base/'wolf-book';out=root/'docs/audit/bs73-evidence'
old=[json.loads(x) for x in (base/'wl70-units.jsonl').read_text().splitlines()]
rows=json.loads((out/'census.json').read_text());lookup={x['file']:x for x in rows}
lines=['exercise\twl70 status\twl70 exit\tcheck directive\tbs73 native status\tbs73 native exit\tbook rig\tcause / disposition\tquoted native diagnostic']
causes={
'exB-4':'E1001 use after move; deliberate lupin trap, retain.',
'exB-5':'E1002 overlapping mut paths; deliberate lupin exclusivity trap, retain.',
'exB-6':'E0804 missing mut receiver in frozen-pool trap demonstration; lupin serves the declared trap, retain.',
'exB-7':'E0804 pool receiver modes in stale-handle trap demonstration; lupin serves the declared trap, retain.',
'ex7-4':'E1001 struct moves on assignment; deliberate lupin trap, retain.',
'ex8-5':'E1005 transfer of an open region; deliberate lupin region-fault trap, retain.',
'ex8-10':'E0804 pool receiver mode in frozen-pool trap demonstration; lupin serves the declared trap, retain.',
'ex9-15':'E0415/E0401 byte-store inference; interpreter-only numeric-pointer exercise, native non-gating probe, retain.',
'ex10-4':'E0401 closure int/() mismatch; deterministic deadlock is a lupin claim, native non-gating probe, retain.',
'ex16-3':'E1001 region used after move; deliberate lupin trap, retain.',
'ex3-13':'Native conservatism for break with a value; explicit lupin-only loop-values exercise, retain.',
'ex9-7a':'Native conservatism for raw pointer indexing; explicit lupin-only exercise, retain.',
'ex9-7b':'Native conservatism for raw pointer indexing; lupin UB oracle is the claim, retain.',
'ex9-14':'Native conservatism for raw pointer indexing; lupin UB oracle is the claim, retain.',
}
for x in old:
 if x['status'] in ('pass','expects-fail'):continue
 rel=x['unit'].removeprefix('fixture:');r=lookup[rel];claim=r['claim'];stem=Path(rel).stem;rig='PASS'
 if stem=='ex5-8':reason='Existing pending row: bool comparator and unspellable take; lupin std-root debt (#45/#58), retain.';rig='PENDING'
 elif stem=='ex8-7':reason='Existing pending row: compiler handle Eq and lupin pool-handle place writes (#60), retain.';rig='PENDING'
 elif stem in causes:reason=causes[stem]
 elif claim.startswith('lupin-run(exit=trap(ub)'):reason='UB oracle belongs to lupin; native arbitrary bytes/status are not a UB verdict, retain.'
 elif claim.startswith('lupin-run(exit=trap(deadlock)'):reason='Lupin deterministic deadlock trap; native is allowed to park, retain.'
 elif stem.startswith('ex28-'):reason='Native conservatism for deeply nested copy; explicit lupin-only recursive-tree exercise, retain.'
 elif claim=='run(exit=nonzero)':reason='Book rig parses nonzero and verifies a built program; wollf verifier cannot parse it (wollf#43), retain.'
 else:raise ValueError(rel)
 quote=next((s for s in r['stderr'].splitlines() if s.startswith('error[') or 'cannot compile this yet' in s),'none; program ran or timed out')
 lines.append('\t'.join([rel,x['status'],str(x['rc']),claim,r['status'],str(r['exit']),rig,reason,quote]))
(out/'exercise-table.tsv').write_text('\n'.join(lines)+'\n')
(out/'wl70-comparison.json').write_text(json.dumps(old,indent=2)+'\n')
(out/'measurement-summary.txt').write_text('Control 591cd5d0779764b7bf043ebe712ec52e1e356aa5\n'
 +'wl70 scope: '+str(len(old))+' fixtures; '+str(dict(collections.Counter(x['status'] for x in old)))+'\n'
 +'bs73 direct native scope: '+str(len(rows))+' exercise roots; '+str(dict(collections.Counter(x['status'] for x in rows)))+'\n'
 +'The full book rig covers all 252 exercise roots and 285 book blocks.\n'
 +'All 33 wl70 anomalies are present in the fresh census. Native UB statuses need not repeat.\n'
 +'Eleven anomaly rows exit 2; seven other fail rows compare native UB execution with a lupin oracle.\n'
 +'All ten refusals repeat; the additional raw refusal is the existing pending ch07/ex7-5.\n'
 +'Remaining direct exits 2 are deliberate fail directives (including five package roots).\n'
 +'No new exercise failure: rig 534 pass, 3 existing pending, 0 fail, 0 flips.\n')
print('table anomaly rows',len(lines)-1)
