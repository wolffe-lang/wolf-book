# Glossary

One term per concept. Each entry names the section that defines it, and
the definition here is the one the book uses everywhere else. Where a
word also has an ordinary English sense that the book cannot avoid, the
entry says so.

**border** (8.4): every edge that leaves one region and enters another.
The border is what the compiler checks; the interior is unchecked because
it does not need checking.

**channel** (10.1, taken apart in 12.1): a typed route between tasks or
procs. A send and the matching receive are one synchronization event.

**closure** (4.2): a function value that carries part of its
environment. Captures are copies, `imm` shares, or region moves, never
mutable windows onto the enclosing function's locals.

**comptime** (18.0): wolf, evaluated during compilation. One tier, one
language, no macro expander.

**error kernel** (15.2): the part of a system whose failure is not
recoverable, kept small on purpose.

**externally sequential** (11.1): a function that has joined everything
it started before it returns, so a caller need not know that concurrency
happened.

**handle** (8.4): an index into a pool carrying the generation its slot
held when the handle was issued. A stale handle is a fault, never a
dangling read.

**imm** (8.5): the mode frozen data has: immutable, shareable from
anywhere, permanent. There is no unfreeze.

**iterator** (5.2): a value that yields elements one at a time. Wolf's
iterators spell their unit: `words()`, `lines()`, `pairs()`.

**join** (10.1): the wait a scope performs at its closing brace for
every task it spawned. One arrow in, one arrow out.

**mode** (7.3): what a parameter says about the caller's value: read for
the duration of the call, `mut` for exclusive access, `take` for
ownership.

**module** (22.0): a directory. Files inside it are invisible to
importers, and the package root is the entry file's directory.

**move** (7.2): handing a value over. The source place is uninitialized
afterward, and reading it is a static error or a trap.

**open row** (6.1): a row ending in `..`: these tags, and possibly more.

**package** (23.0): a directory outside your project, with its own
manifest; contrast *module*.

**place** (7.1): any path the language can name and the compiler can
prove distinct from another: a variable, a field, a field of a field.

**proc** (14.0): a failure domain with its own memory, whose death is an
event other procs can subscribe to.

**provenance** (9.3): the lineage a pointer inherits its permission
from, distinct from its address.

**region** (8.1): a checked arena: allocations that live and die
together, with the compiler proving nothing escapes alive. "Arena" in
this book is always the C construct a region is compared to.

**rendezvous** (30.3): a zero-capacity channel, where a send blocks
until a receiver takes the value.

**ring** (9.1): a place in the source where the proof obligation changes
hands. Wolf has exactly three.

**row** (6.1): the set of tags a function may fail with, written between
braces after `!` and complete. Part 1's running example prints a receipt
whose lines are also called rows; the error row is always the one with
braces.

**scope** (10.1): the block that owns one or more tasks and joins them
at its closing brace. The word also has its ordinary lexical sense, and
the book uses it that way where no task is involved.

**sentinel-int convention** (26.2): C's habit of making one integer
value mean "no value". The two K&R twins are built from it, and wolf's
rows are what replace it.

**strategy** (8.3): how a region's interior is managed, chosen where the
region is made and nowhere else.

**tag** (9.3): an identity a pointer value carries, distinct from its
address. Also, in a row, the name of one way a function can fail; the two
senses never appear on the same page.

**task** (10.1): a unit of concurrent work, owned by a scope;
distinct from an OS thread.

**trap** (3.3): the fault of a defined execution: the program was legal,
it ran, and it hit a rule the language enforces at run time, in every
build profile. Appendix B closes the set at twelve kinds.

**view** (2.5): two machine words, a pointer and a length, aimed at
bytes that already exist. Taking one copies nothing.

**view set** (7.5): a promise in a signature about which paths a method
touches: `mut self.{title, subtitle}`.

**witness** (18.1): a comptime `assert` whose failure is a compile
error. A witness is how a compile-time fact gets a location in your
source.
