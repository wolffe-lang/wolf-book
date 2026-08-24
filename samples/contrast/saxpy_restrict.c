/* saxpy_restrict --- the aliasing chapter's kernel, C side, promises on.
 *
 * An ORIGINAL implementation of the BLAS-folklore saxpy shape, written
 * for chapter 21's side-by-side. `restrict` is the programmer's word
 * that xs and ys do not overlap: the compiler may plan on it, and if the
 * word is false the program is undefined. Nothing checks it, which is
 * the page's whole subject.
 */
#include <stdio.h>
#include <stddef.h>

void saxpy(double a, const double *restrict xs, double *restrict ys, size_t n) {
    for (size_t i = 0; i < n; i++) ys[i] = a * xs[i] + ys[i];
}

int main(void) {
    double xs[5] = { 1.0, 2.0, 3.0, 4.0, 5.0 };
    double ys[5] = { 10.0, 10.0, 10.0, 10.0, 10.0 };
    saxpy(2.0, xs, ys, 5);
    printf("%g %g\n", ys[0], ys[4]);
    return 0;
}
