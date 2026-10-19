# bs69, sourced on hasu (its gh has no valid login): fetch <dir> <repo> <tag> <asset> — the digest from the
# PUBLIC release API (unauthenticated curl), the asset from its public download URL, compared, unpacked on
# MATCH only; fetch2 the second read into its own directory, `sha256sum -c`, the trees compared (diff -r).
apidigest() { curl -fsSL "https://api.github.com/repos/wolffe-lang/$1/releases/tags/$2" | python3 -c 'import json,sys; a=sys.argv[1]; d=json.load(sys.stdin); print([x["digest"] for x in d["assets"] if x["name"]==a][0].split(":")[1])' "$3"; }
fetch() { local d=$1 r=$2 t=$3 a=$4
  local want=$(apidigest $r $t $a); [ -n "$want" ] || { echo "NO DIGEST $a"; return 1; }
  curl -fsSL -o $d/$a "https://github.com/wolffe-lang/$r/releases/download/$t/$a" || { echo "DOWNLOAD FAILED $a"; return 1; }
  local got=$(sha256sum $d/$a | cut -c1-64)
  echo "$a api=$want got=$got $( [ "$want" = "$got" ] && echo MATCH || echo MISMATCH)"
  [ "$want" = "$got" ] && tar -C $d -xzf $d/$a && rm -f $d/$a; }
fetch2() { local d=$1 r=$2 t=$3 a=$4 first=$5; local d2=$d/.second; rm -rf $d2; mkdir -p $d2
  local want=$(apidigest $r $t $a)
  curl -fsSL -o $d2/$a "https://github.com/wolffe-lang/$r/releases/download/$t/$a" || { echo "SECOND DOWNLOAD FAILED $a"; return 1; }
  (cd $d2 && echo "$want  $a" | sha256sum -c -) && tar -C $d2 -xzf $d2/$a && rm -f $d2/$a
  local n=$(basename $first)
  if diff -r $first $d2/$n > /dev/null; then echo "$a second read: tree $n IDENTICAL"; else echo "$a second read: tree $n DIFFERS"; fi
  rm -rf $d2; }
