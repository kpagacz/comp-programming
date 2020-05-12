#include<stdio.h>
#include<stdlib.h>

/* gets WA, but I don't know why */

int main(void) {
    int tests;
    scanf("%i", &tests);

    long first_age, first_position, second_age, second_position, positions_between;

    for(int i = 0; i < tests; i++) {
        scanf("%li %li %li %li", &first_age, &first_position, &second_age, &second_position);
        positions_between = abs(second_position - first_position) - 1;
        printf("%li ", positions_between);
        if(positions_between % 3 == 0) {
            (first_age > second_age) ? printf("0\n") : printf("-1\n");
        } else {
            (first_age > second_age) ? printf("-1\n") : printf("0\n");
        }

    }
}