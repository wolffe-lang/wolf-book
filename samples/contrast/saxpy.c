/* saxpy --- the same kernel with the promise withheld.
 *
 * An ORIGINAL implementation for chapter 21's exercise 21-1: no
 * `restrict`, so the C compiler must plan for xs and ys overlapping.
 * The program's behavior is identical on these inputs; what changed is
 * what the optimizer is allowed to assume.
 */
#include <stdio.h>
#include <stddef.h>

void saxpy(double a, const double *xs, double *ys, size_t n) {
    for (size_t i = 0; i < n; i++) ys[i] = a * xs[i] + ys[i];
}

int main(void) {
    double xs[5] = { 1.0, 2.0, 3.0, 4.0, 5.0 };
    double ys[5] = { 10.0, 10.0, 10.0, 10.0, 10.0 };
    saxpy(2.0, xs, ys, 5);
    printf("%g %g\n", ys[0], ys[4]);
    return 0;
}
