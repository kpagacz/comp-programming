#include<stdio.h>
#include<math.h>

/* this gets WA, but almost the same code in c++ gets AC, so I think there is an
issue with my rounding function in here */

int main(void) {
    int tests;
    scanf("%i", &tests);
    double r1, r2;

    for(int i = 0; i < tests; i++) {
        scanf("%lf %lf", &r1, &r2);
        printf("%.2lf\n", roundf(r1 * r2 * 200) / 100);
    }    
    
    return 0;
}