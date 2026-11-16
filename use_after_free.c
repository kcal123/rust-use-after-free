#include <stdio.h>
#include <stdlib.h>

int main(void) {
    int *p = malloc(sizeof(int));
    if (p == NULL) return 1;

    *p = 42;
    int *q = p;

    free(p);

    // Intentional use-after-free: q still points to the freed allocation.
    printf("%d\n", *q);
    return 0;
}
