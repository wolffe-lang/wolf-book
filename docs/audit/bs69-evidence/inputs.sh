#!/bin/bash
# bs69 (from bs68): re-derive the contract's inputs from origin (read only; the org checkouts' objects).
set -u
O=/Users/mfwolffe/GithubOrgs/wolffe-lang
WL=$O/wolf-lang; WI=$O/wolf-interp; WS=$O/wolf-std; WP=$O/wolf
echo "date $(date -u +%FT%TZ)"
echo "== tags"
for t in v0.2.32 v0.2.33; do echo "wolf-lang $t -> $(git -C $WL rev-parse ${t}^{}) (tag object $(git -C $WL rev-parse $t | cut -c1-8), $(git -C $WL cat-file -t $t))"; done
for t in v0.1.54; do echo "wolf-interp $t -> $(git -C $WI rev-parse ${t}^{}) ($(git -C $WI cat-file -t $t))"; done
echo "== releases (API)"
for rt in "wolf-lang v0.2.33" "wolf-lang v0.2.32" "wolf-interp v0.1.54"; do set -- $rt
  gh api repos/wolffe-lang/$1/releases/tags/$2 --jq '"\(.tag_name) release \(.id) draft=\(.draft) published \(.published_at)\n" + ([.assets[] | "  \(.name) \(.size) \(.digest)"] | join("\n"))'; done
echo "== PAIRING / STD-PIN"
for t in v0.2.32 v0.2.33; do echo "$t: $(git -C $WL show ${t}:crates/wolf_driver/PAIRING | grep -v '^#' | tr '\n' ' ') $(git -C $WL show ${t}:crates/wolf_driver/STD-PIN | grep -v '^#')"; done
echo "== counts"
echo "v0.2.32..v0.2.33 $(git -C $WL rev-list --count v0.2.32..v0.2.33)"
echo "294d626..v0.2.33 $(git -C $WL rev-list --count 294d626..v0.2.33); 294d626..v0.2.32 $(git -C $WL rev-list --count 294d626..v0.2.32)"
echo "releases after 294d626 (v0.2.24): $(git -C $WL tag --contains 294d626 | grep -E '^v0\.2\.[0-9]+$' | sort -V | grep -v '^v0.2.24$' | tr '\n' ' ')"
echo "lupin vendor/upstream/PIN at v0.1.54: $(git -C $WI show v0.1.54:vendor/upstream/PIN | head -1)"
echo "== spec"
an() { git -C $WL show ${1}:spec/anchors.json | python3 -c 'import json,sys; d=json.load(sys.stdin); a=d.get("anchors",d); print("\n".join(sorted(a)))'; }
echo "anchors v0.2.32 $(an v0.2.32 | wc -l) v0.2.33 $(an v0.2.33 | wc -l)"
diff <(an v0.2.32) <(an v0.2.33) | grep '^[<>]'
for p in spec/grammar.ebnf crates/wolf_driver/src/help.rs docs/diagnostics.md; do echo "$p v0.2.32..v0.2.33: $(git -C $WL diff --stat v0.2.32 v0.2.33 -- $p | tail -1)"; done
echo "== prelude names v0.2.32..v0.2.33"
pn() { git -C $WL show ${1}:spec/prelude.json | python3 -c 'import json,sys; d=json.load(sys.stdin); print("\n".join(sorted(x if isinstance(x,str) else x.get("name","?") for x in (d.get("functions") or d.get("names") or d))))' 2>&1; }
diff <(pn v0.2.32) <(pn v0.2.33) | grep '^[<>]'
echo "== std"
for c in d37c911 cceda63; do echo "wolf-std $c -> $(git -C $WS rev-parse $c) ($(git -C $WS cat-file -t $c))"; done
echo "std/ d37c911..cceda63:"; git -C $WS diff --stat d37c911 cceda63 -- std | tail -5
echo "wolf-std origin/trunk: $(git -C $WS rev-parse origin/trunk)"
echo "== planning"; echo "wolf origin/trunk $(git -C $WP rev-parse origin/trunk)"
echo "== wolf-book"; git -C $O/wolf-book rev-parse origin/trunk
