# bs66 (from bs64), sourced: fetch <dir> <repo> <tag> <asset> — the digest from the release API,
# download, compare, unpack on MATCH only. Portable: sha256sum on linux, shasum -a 256 on macOS.
sha() { if command -v sha256sum >/dev/null; then sha256sum "$1"; else shasum -a 256 "$1"; fi | awk '{print $1}'; }
fetch() { local d=$1 r=$2 t=$3 a=$4
  local want=$(gh api repos/wolffe-lang/$r/releases/tags/$t --jq ".assets[] | select(.name==\"$a\") | .digest" | sed 's/^sha256://')
  [ -n "$want" ] || { echo "NO DIGEST $a"; return 1; }
  (cd $d && gh release download $t -R wolffe-lang/$r -p "$a" --clobber) || { echo "DOWNLOAD FAILED $a"; return 1; }
  local got=$(sha $d/$a)
  echo "$a api=$want got=$got $( [ "$want" = "$got" ] && echo MATCH || echo MISMATCH)"
  [ "$want" = "$got" ] && tar -C $d -xzf $d/$a && rm -f $d/$a; }
# the host's archive triple
triple() { case "$(uname -sm)" in "Linux x86_64") echo x86_64-unknown-linux-gnu;; "Darwin arm64") echo aarch64-apple-darwin;; *) echo UNKNOWN;; esac; }
