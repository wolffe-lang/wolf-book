#!/bin/bash
# bs70 (from bs69): re-derive the contract's inputs from origin (read only; the org checkouts' objects).
set -u
O=/Users/mfwolffe/GithubOrgs/wolffe-lang
WL=$O/wolf-lang; WI=$O/wolf-interp; WS=$O/wolf-std; WP=$O/wolf
echo "date $(date -u +%FT%TZ)"
echo "== tags"
for t in v0.2.33 v0.2.34; do echo "wolf-lang $t -> $(git -C $WL rev-parse ${t}^{}) (tag object $(git -C $WL rev-parse $t | cut -c1-8), $(git -C $WL cat-file -t $t))"; done
for t in v0.1.54 v0.1.55; do echo "wolf-interp $t -> $(git -C $WI rev-parse ${t}^{}) (tag object $(git -C $WI rev-parse $t | cut -c1-8), $(git -C $WI cat-file -t $t))"; done
echo "== releases (API)"
for rt in "wolf-lang v0.2.34" "wolf-lang v0.2.33" "wolf-interp v0.1.55" "wolf-interp v0.1.54"; do set -- $rt
  gh api repos/wolffe-lang/$1/releases/tags/$2 --jq '"\(.tag_name) release \(.id) draft=\(.draft) published \(.published_at)\n" + ([.assets[] | "  \(.name) \(.size) \(.digest)"] | join("\n"))'; done
echo "== PAIRING / STD-PIN"
for t in v0.2.33 v0.2.34; do echo "$t: $(git -C $WL show ${t}:crates/wolf_driver/PAIRING | grep -v '^#' | tr '\n' ' ') $(git -C $WL show ${t}:crates/wolf_driver/STD-PIN | grep -v '^#')"; done
echo "== counts"
echo "v0.2.33..v0.2.34 $(git -C $WL rev-list --count v0.2.33..v0.2.34)"
echo "294d626..v0.2.34 $(git -C $WL rev-list --count 294d626..v0.2.34); 294d626..v0.2.33 $(git -C $WL rev-list --count 294d626..v0.2.33)"
echo "294d626 is an ancestor of v0.2.34: $(git -C $WL merge-base --is-ancestor 294d626 v0.2.34 && echo yes || echo NO)"
echo "releases after 294d626 (v0.2.24): $(git -C $WL tag --contains 294d626 | grep -E '^v0\.2\.[0-9]+$' | sort -V | grep -v '^v0.2.24$' | tr '\n' ' ')"
echo "v0.1.54..v0.1.55 $(git -C $WI rev-list --count v0.1.54..v0.1.55)"
for t in v0.1.54 v0.1.55; do echo "lupin vendor/upstream/PIN at $t: $(git -C $WI show ${t}:vendor/upstream/PIN | head -1)"; done
echo "src/eval/placetrace.rs v0.1.54..v0.1.55: $(git -C $WI diff --quiet v0.1.54 v0.1.55 -- src/eval/placetrace.rs && echo IDENTICAL || echo MOVED)"
echo "lupin src v0.1.54..v0.1.55:"; git -C $WI diff --stat v0.1.54 v0.1.55 -- src | tail -4
echo "== spec"
an() { git -C $WL show ${1}:spec/anchors.json | python3 -c 'import json,sys; d=json.load(sys.stdin); a=d.get("anchors",d); print("\n".join(sorted(a)))'; }
echo "anchors v0.2.33 $(an v0.2.33 | wc -l) v0.2.34 $(an v0.2.34 | wc -l)"
diff <(an v0.2.33) <(an v0.2.34) | grep '^[<>]'
for p in spec/grammar.ebnf crates/wolf_driver/src/help.rs docs/diagnostics.md; do echo "$p v0.2.33..v0.2.34: $(git -C $WL diff --stat v0.2.33 v0.2.34 -- $p | tail -1)"; done
echo "== prelude names v0.2.33..v0.2.34"
pn() { git -C $WL show ${1}:spec/prelude.json | python3 -c 'import json,sys; d=json.load(sys.stdin); print("\n".join(sorted([("name " + (x if isinstance(x,str) else x.get("name","?"))) for x in d["names"]] + [("mark " + (x if isinstance(x,str) else x.get("name","?"))) for x in d["marks"]])))'; }
echo "v0.2.33 $(pn v0.2.33 | wc -l) entries, v0.2.34 $(pn v0.2.34 | wc -l)"
diff <(pn v0.2.33) <(pn v0.2.34) | grep '^[<>]'
echo "== std"
for c in cceda63 e036bec 3bacc5f; do echo "wolf-std $c -> $(git -C $WS rev-parse $c) ($(git -C $WS cat-file -t $c))"; done
echo "std/ cceda63..e036bec:"; git -C $WS diff --stat cceda63 e036bec -- std | tail -6
echo "std/term at e036bec: $(git -C $WS show e036bec:std/term/term.lu | grep -c 'raw_cr') mentions of raw_cr; at 3bacc5f: $(git -C $WS show 3bacc5f:std/term/term.lu | grep -c 'raw_cr')"
echo "== planning"; echo "wolf HEAD $(git -C $WP rev-parse HEAD); the contract last touched at $(git -C $WP log -1 --format=%h -- sprints/book/70-the-book-at-0234/)"
echo "== wolf-book"; git ls-remote https://github.com/wolffe-lang/wolf-book.git trunk
