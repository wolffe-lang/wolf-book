/* arena_nodes --- ten thousand nodes under malloc discipline.
 *
 * An ORIGINAL program for chapter 21's arena side-by-side: build a
 * pass's worth of nodes, read them once, tear them down. Every node is
 * a retail allocation and a retail free — 20,000 allocator interactions
 * for one pass — and the correctness of the teardown is a code review's
 * hope, not a checker's proof. The wolf twin on the page does the same
 * work inside one region.
 */
#include <stdio.h>
#include <stdlib.h>

struct node {
    long value;
    long weight;
};

int main(void) {
    struct node *nodes[10000];
    long total = 0;
    for (int i = 0; i < 10000; i++) {
        nodes[i] = malloc(sizeof(struct node));
        if (nodes[i] == NULL)
            return 1;
        nodes[i]->value = i;
        nodes[i]->weight = i % 7;
    }
    for (int i = 0; i < 10000; i++)
        total += nodes[i]->weight;
    for (int i = 0; i < 10000; i++)
        free(nodes[i]);
    printf("%ld\n", total);
    return 0;
}
