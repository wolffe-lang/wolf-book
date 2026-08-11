/* wordtree --- count the occurrences of each input word, alphabetized.
 *
 * An ORIGINAL implementation, written after the manner of Kernighan and
 * Ritchie, "The C Programming Language", 2nd ed., section 6.5. The
 * addtree/treeprint/talloc division of labor and the recursive in-order
 * walk are their teaching shape; the code below is ours, and no listing
 * from that book is reproduced here. See PERMISSIONS.md.
 *
 * The side-by-side's subject is allocation. Every node here is a
 * separate malloc, every word is a second one, every one of them can
 * fail, and the program owns a matching free for each --- treefree at
 * the bottom of this file exists only because malloc was called at the
 * top. The original stops before writing it, which is honest about
 * chapter 6's scope and is also how the leak gets into production.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define MAXWORD 100

struct tnode {                  /* one distinct word */
    char *word;                 /* the text, separately allocated */
    int count;                  /* how many times it has been seen */
    struct tnode *left;         /* words alphabetically before it */
    struct tnode *right;        /* words alphabetically after it */
};

static struct tnode *addtree(struct tnode *p, const char *w);
static void          treeprint(const struct tnode *p);
static void          treefree(struct tnode *p);
static struct tnode *talloc(void);
static char         *dupstr(const char *s);
static int           getword(char *word, int lim);

/* Every allocating routine can fail, so every caller has to look. The
 * flag is how a recursive function reports a failure it cannot return. */
static int nomem = 0;

/* Add w to the tree at p, or bump its count. Returns the (possibly new)
 * subtree root. On allocation failure it sets `nomem` and returns what
 * it was given --- the only channel a pointer-returning function has. */
static struct tnode *addtree(struct tnode *p, const char *w)
{
    int cond;

    if (p == NULL) {            /* a word not seen before */
        p = talloc();
        if (p == NULL) {
            nomem = 1;
            return NULL;
        }
        p->word = dupstr(w);
        if (p->word == NULL) {
            free(p);            /* the half-built node must not leak */
            nomem = 1;
            return NULL;
        }
        p->count = 1;
        p->left = NULL;
        p->right = NULL;
    } else if ((cond = strcmp(w, p->word)) == 0)
        ++p->count;             /* repeated word */
    else if (cond < 0)
        p->left = addtree(p->left, w);
    else
        p->right = addtree(p->right, w);
    return p;
}

/* In-order walk: left subtree, this node, right subtree. Alphabetical
 * because the tree was built by strcmp. */
static void treeprint(const struct tnode *p)
{
    if (p != NULL) {
        treeprint(p->left);
        printf("%4d %s\n", p->count, p->word);
        treeprint(p->right);
    }
}

/* The other half of every malloc above. Post-order, because a node's
 * children must be freed before the node that points at them. */
static void treefree(struct tnode *p)
{
    if (p != NULL) {
        treefree(p->left);
        treefree(p->right);
        free(p->word);
        free(p);
    }
}

static struct tnode *talloc(void)
{
    return (struct tnode *) malloc(sizeof(struct tnode));
}

static char *dupstr(const char *s)
{
    char *p;

    p = (char *) malloc(strlen(s) + 1);
    if (p != NULL)
        strcpy(p, s);
    return p;
}

/* Read the next alphabetic word into word[0..lim-1]. Returns the first
 * character of the word, EOF, or the non-word character it found. */
static int getword(char *word, int lim)
{
    int c;
    char *w;

    w = word;
    while ((c = getchar()) != EOF && !isalpha(c) && c != '\n')
        ;
    if (c == EOF) {
        *w = '\0';
        return EOF;
    }
    if (!isalpha(c)) {
        *w++ = (char) c;
        *w = '\0';
        return c;
    }
    *w++ = (char) tolower(c);
    for (; --lim > 1; w++) {
        c = getchar();
        if (!isalpha(c)) {
            if (c != EOF)
                ungetc(c, stdin);
            break;
        }
        *w = (char) tolower(c);
    }
    *w = '\0';
    return word[0];
}

int main(void)
{
    struct tnode *root;
    char word[MAXWORD];
    int t;

    root = NULL;
    while ((t = getword(word, MAXWORD)) != EOF) {
        if (isalpha(t))
            root = addtree(root, word);
        if (nomem) {
            fprintf(stderr, "wordtree: out of memory\n");
            treefree(root);
            return 1;
        }
    }
    treeprint(root);
    treefree(root);             /* the closing brace C does not have */
    return 0;
}
