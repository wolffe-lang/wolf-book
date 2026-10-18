#!/bin/bash
# bs68: re-derive the contract's inputs from origin (read only; blobless reads of the org checkouts' objects).
set -u
O=/Users/mfwolffe/GithubOrgs/wolffe-lang
WL=$O/wolf-lang; WI=$O/wolf-interp; WS=$O/wolf-std
echo "date $(date -u +%FT%TZ)"
echo "== tags"
for t in v0.2.30 v0.2.31 v0.2.32; do echo "wolf-lang $t -> $(git -C $WL rev-parse ${t}^{}) (tag object $(git -C $WL rev-parse $t | cut -c1-8), $(git -C $WL cat-file -t $t))"; done
for t in v0.1.53 v0.1.54; do echo "wolf-interp $t -> $(git -C $WI rev-parse ${t}^{}) ($(git -C $WI cat-file -t $t))"; done
echo "== PAIRING / STD-PIN"
for t in v0.2.30 v0.2.31 v0.2.32; do echo "$t: $(git -C $WL show ${t}:crates/wolf_driver/PAIRING | grep -v '^#' | tr '\n' ' ') $(git -C $WL show ${t}:crates/wolf_driver/STD-PIN | grep -v '^#')"; done
echo "== counts"
echo "v0.2.30..v0.2.32 $(git -C $WL rev-list --count v0.2.30..v0.2.32); v0.2.30..v0.2.31 $(git -C $WL rev-list --count v0.2.30..v0.2.31); v0.2.31..v0.2.32 $(git -C $WL rev-list --count v0.2.31..v0.2.32)"
echo "294d626..v0.2.32 $(git -C $WL rev-list --count 294d626..v0.2.32); 294d626..v0.2.30 $(git -C $WL rev-list --count 294d626..v0.2.30)"
echo "releases after 294d626 (v0.2.24): $(git -C $WL tag --contains 294d626 | grep -E '^v0\.2\.[0-9]+$' | sort -V | grep -v '^v0.2.24$' | tr '\n' ' ')"
echo "v0.1.53..v0.1.54 $(git -C $WI rev-list --count v0.1.53..v0.1.54)"
echo "placetrace.rs v0.1.53..v0.1.54: $(git -C $WI diff --stat v0.1.53 v0.1.54 -- src/eval/placetrace.rs | tail -1)(empty = byte-identical)"
echo "lupin vendor/upstream/PIN at v0.1.54: $(git -C $WI show v0.1.54:vendor/upstream/PIN | head -1)"
echo "== spec"
an() { git -C $WL show ${1}:spec/anchors.json | python3 -c 'import json,sys; d=json.load(sys.stdin); a=d.get("anchors",d); print("\n".join(sorted(a)))'; }
echo "anchors v0.2.30 $(an v0.2.30 | wc -l) v0.2.32 $(an v0.2.32 | wc -l)"
diff <(an v0.2.30) <(an v0.2.32) | grep '^[<>]'
for p in spec/grammar.ebnf crates/wolf_driver/src/help.rs; do echo "$p v0.2.30..v0.2.32: $(git -C $WL diff --stat v0.2.30 v0.2.32 -- $p | tail -1)"; done
echo "== std"
for c in fd6bcdc d37c911 dcf3e89; do echo "wolf-std $c -> $(git -C $WS rev-parse $c) ($(git -C $WS cat-file -t $c))"; done
echo "std/ fd6bcdc..d37c911:"; git -C $WS diff --stat fd6bcdc d37c911 -- std | tail -30
echo "std/ d37c911..dcf3e89 (trunk, NOT in any release):"; git -C $WS diff --stat d37c911 dcf3e89 -- std | tail -15
echo "== wolf-book"; git -C ~/lanes/bs68/wolf-book rev-parse origin/trunk
