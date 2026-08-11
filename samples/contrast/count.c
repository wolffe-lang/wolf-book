/* count --- lines, words, and bytes, per file and in total.
 *
 * An ORIGINAL implementation, written after the manner of Kernighan and
 * Ritchie, "The C Programming Language", 2nd ed., sections 1.5-1.6. The
 * character loop and the IN/OUT word-state machine are their teaching
 * shape; the code below is ours, and no listing from that book is
 * reproduced here. See PERMISSIONS.md.
 *
 * Two conventions in here are the side-by-side's whole subject:
 *   - the state machine is spelled as two #defines and an int;
 *   - failure is a sentinel int, and the caller has to remember to look.
 */
#include <stdio.h>

#define IN  1   /* inside a word */
#define OUT 0   /* between words */

/* Three numbers, so a struct: chapter 1 would have used three globals,
 * but a per-file row and a running total need two of them at once. */
struct tally {
    long lines;
    long words;
    long bytes;
};

/* Count one open stream into *t.
 *
 * Returns 0, or -1 if the stream errored. This is the sentinel-int
 * convention: the value -1 carries no detail, it shares a type with
 * every legitimate answer, and nothing in the signature obliges the
 * caller to test it. */
static int count(FILE *fp, struct tally *t)
{
    int c, state;

    state = OUT;
    t->lines = 0;
    t->words = 0;
    t->bytes = 0;
    while ((c = getc(fp)) != EOF) {
        ++t->bytes;
        if (c == '\n')
            ++t->lines;
        if (c == ' ' || c == '\n' || c == '\t')
            state = OUT;
        else if (state == OUT) {
            state = IN;
            ++t->words;
        }
    }
    return ferror(fp) ? -1 : 0;
}

static void row(const struct tally *t, const char *name)
{
    printf("%8ld%8ld%8ld %s\n", t->lines, t->words, t->bytes, name);
}

static void add(struct tally *total, const struct tally *t)
{
    total->lines += t->lines;
    total->words += t->words;
    total->bytes += t->bytes;
}

int main(int argc, char *argv[])
{
    struct tally t, total;
    FILE *fp;
    int i, files, failed;

    total.lines = 0;
    total.words = 0;
    total.bytes = 0;
    failed = 0;
    files = 0;

    if (argc == 1) {                /* no names: the stream is stdin */
        if (count(stdin, &t) < 0) {
            fprintf(stderr, "count: error reading standard input\n");
            return 1;
        }
        row(&t, "-");
        return 0;
    }

    for (i = 1; i < argc; ++i) {
        if ((fp = fopen(argv[i], "r")) == NULL) {
            fprintf(stderr, "count: cannot open %s\n", argv[i]);
            failed = 1;
            continue;
        }
        if (count(fp, &t) < 0) {
            fprintf(stderr, "count: error reading %s\n", argv[i]);
            failed = 1;
        } else {
            row(&t, argv[i]);
            add(&total, &t);
            ++files;
        }
        fclose(fp);
    }
    if (files > 1)
        row(&total, "total");
    return failed;
}
