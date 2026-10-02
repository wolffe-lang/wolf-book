// bs56: classify a gate records status faults by code; for 304s apply the gates own title/number test to the fault line.
const fs = require("fs"); const SUFFIX = " - The Wolf Book";
const LINE = /^(\S+) -> "(.*)": HTTP (\d+|null) (\S+) title="(.*)" h1="(.*)"$/;
const parse = (l) => { const m = /^(\d+)\.\s+(.*)$/.exec(l); return m ? [m[1], m[2]] : [null, l]; };
for (const path of process.argv.slice(2)) for (const r of JSON.parse(fs.readFileSync(path, "utf8"))) {
  const by = {}; let right = 0; const wrong = [];
  for (const f of r.faults.status) { const m = LINE.exec(f); const code = m ? m[3] : "unparsed"; by[code] = (by[code] || 0) + 1;
    if (m && code === "304") { const [num, name] = parse(m[2]); const h1num = parse(m[6])[0]; if (m[5] === name + SUFFIX && h1num === num) right++; else wrong.push(f); } }
  const other = Object.fromEntries(Object.entries(r.faults).filter(([k, v]) => k !== "status" && v.length).map(([k, v]) => [k, v.length]));
  console.log(`${r.engine} ${r.viewport}: clicks ${r.clicks}, at200 ${r.landed200}, at304 ${r.landed304 ?? "-"}, status-by-code ${JSON.stringify(by)}, 304-landed-right ${right}, 304-wrong ${wrong.length}, other ${JSON.stringify(other)}`);
  for (const f of wrong.slice(0, 5)) console.log("   WRONG", f);
  for (const k of ["click", "entries", "state", "load", "search", "title", "number"]) for (const f of r.faults[k].slice(0, 3)) console.log(`   ${k}: ${f}`);
}
