# sourced: fetch <dir> <repo> <tag> <asset> — digest from the release API, download, compare, unpack on MATCH
fetch() { local d=$1 r=$2 t=$3 a=$4
  local want=$(gh api repos/wolffe-lang/$r/releases/tags/$t --jq ".assets[] | select(.name==\"$a\") | .digest" | sed 's/^sha256://')
  [ -n "$want" ] || { echo "NO DIGEST $a"; return 1; }
  (cd $d && gh release download $t -R wolffe-lang/$r -p "$a" --clobber) || { echo "DOWNLOAD FAILED $a"; return 1; }
  local got=$(sha256sum $d/$a | awk '{print $1}')
  echo "$a api=$want got=$got $( [ "$want" = "$got" ] && echo MATCH || echo MISMATCH)"
  [ "$want" = "$got" ] && tar -C $d -xzf $d/$a && rm -f $d/$a; }
