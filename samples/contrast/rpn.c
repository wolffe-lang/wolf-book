/* rpn --- a reverse-Polish calculator.
 *
 * An ORIGINAL implementation, written after the manner of Kernighan and
 * Ritchie, "The C Programming Language", 2nd ed., section 4.3. The
 * getop/push/pop division of labor and the getch/ungetch pushback pair
 * are their teaching shape; the code below is ours, and no listing from
 * that book is reproduced here. See PERMISSIONS.md.
 *
 * The side-by-side's subject is the error path. `pop` has to return a
 * double, and every double is a legitimate answer, so an empty stack
 * returns 0.0 and sets a flag the caller is trusted to consult. The
 * flag is the whole safety mechanism, and nothing enforces it.
 */
#include <stdio.h>
#include <stdlib.h>     /* atof */
#include <ctype.h>

#define MAXOP   100     /* biggest operand or operator */
#define NUMBER  '0'     /* signal that a number was found */
#define MAXVAL  100     /* maximum depth of the value stack */
#define BUFSIZE 100     /* depth of the character pushback */

static int    getop(char s[]);
static void   push(double f);
static double pop(void);
static int    getch(void);
static void   ungetch(int c);

static int    sp = 0;           /* next free stack position */
static double val[MAXVAL];      /* the value stack */

/* The error flag. Set by push on overflow, by pop on underflow, by
 * ungetch on pushback overflow, and by main on division by zero.
 * Consulted by hand, at end of expression, because nothing else will. */
static int broken = 0;

static void push(double f)
{
    if (sp < MAXVAL)
        val[sp++] = f;
    else {
        fprintf(stderr, "rpn: stack full, cannot push %g\n", f);
        broken = 1;
    }
}

/* Returns 0.0 on underflow --- indistinguishable from a pushed zero. */
static double pop(void)
{
    if (sp > 0)
        return val[--sp];
    fprintf(stderr, "rpn: stack empty\n");
    broken = 1;
    return 0.0;
}

static int buf[BUFSIZE];        /* characters read ahead of the parser */
static int bufp = 0;            /* next free slot in buf */

static int getch(void)
{
    return (bufp > 0) ? buf[--bufp] : getchar();
}

static void ungetch(int c)
{
    if (bufp >= BUFSIZE) {
        fprintf(stderr, "rpn: pushback buffer full\n");
        broken = 1;
    } else
        buf[bufp++] = c;
}

/* Read the next operator or numeric operand. */
static int getop(char s[])
{
    int i, c;

    while ((s[0] = c = getch()) == ' ' || c == '\t')
        ;
    s[1] = '\0';
    if (!isdigit(c) && c != '.' && c != '-')
        return c;               /* not a number */
    i = 0;
    if (c == '-') {             /* a sign, or the operator */
        c = getch();
        if (!isdigit(c) && c != '.') {
            ungetch(c);
            return '-';
        }
        s[++i] = (char) c;
    }
    if (isdigit(c))
        while (isdigit(s[++i] = c = getch()))
            ;
    if (c == '.')               /* collect a fractional part */
        while (isdigit(s[++i] = c = getch()))
            ;
    s[i] = '\0';
    if (c != EOF)
        ungetch(c);
    return NUMBER;
}

int main(void)
{
    int type;
    double op2;
    char s[MAXOP];

    while ((type = getop(s)) != EOF) {
        switch (type) {
        case NUMBER:
            push(atof(s));
            break;
        case '+':
            push(pop() + pop());
            break;
        case '*':
            push(pop() * pop());
            break;
        case '-':
            op2 = pop();
            push(pop() - op2);
            break;
        case '/':
            op2 = pop();
            if (op2 == 0.0) {
                fprintf(stderr, "rpn: division by zero\n");
                broken = 1;
                break;
            }
            push(pop() / op2);
            break;
        case '\n':              /* end of an expression */
            if (broken || sp < 1) {
                printf("error\n");
            } else {
                printf("%.4g\n", pop());
            }
            sp = 0;
            broken = 0;
            break;
        default:
            fprintf(stderr, "rpn: unknown command %s\n", s);
            broken = 1;
            break;
        }
    }
    return 0;
}
