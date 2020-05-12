#include<stdio.h>

int main(void) {
    int tests;
    scanf("%i", &tests);

    long stas_w, stas_l, wies_w, wies_l, grzes_w, grzes_l;
    long long jas_w;

    for (int i = 0; i < tests; i ++) {
        scanf(" %li %li %li %li %li %li", &stas_w, &stas_l, &wies_w, &wies_l, &grzes_w, &grzes_l);
        jas_w = 2 * stas_l + stas_w - (wies_w + grzes_w);
        printf("%lli %lli\n", jas_w, stas_l + stas_w - jas_w);
    }

    return 0;
}