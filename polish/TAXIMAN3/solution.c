#include <stdio.h>
#include <stdlib.h>

int main() {
    int dims, tests;
    long int coordinate;
    long long int distance;

    scanf("%i %i", &dims, &tests);

    int* coordinates = malloc(sizeof(int) * 2 * dims);

    for(int i = 0; i < tests; i++) {
        distance = 0;
        // input coordinates
        for(int j = 0; j < 2 * dims; j++) {
            scanf("%li", &coordinate);
            coordinates[j] = coordinate;
        }


        // sum up distance
        for(int j = 0; j < dims; j++) {
            distance += abs(coordinates[j] - coordinates[dims + j]);
        }

        // output
        printf("%lli\n", distance);
    }

    return 0;
}