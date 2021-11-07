#include<stdio.h>

/* something is wrong, but tough for me to debug */

int main(void) {
    int tests;
    scanf("%i", &tests);

    int number_of_cities;
    int connections[100][100], jakubs[100][100], random[100];

    for(int i = 0; i < 100; i++) {
        random[i] = i + 1;
    }

    for(int i = 0; i < tests; i++) {
        /* input dimensions */
        scanf("%i", &number_of_cities);

        /* input connections */
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                scanf("%i", &connections[j][k]);
            }
        }

        /* input jakub */
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                scanf("%i", &jakubs[j][k]);
            }
        }

        /* calculate temp */
        int temp[100] = {0};
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                temp[j] += random[j] * connections[j][k];
            }
        }

        /* compare the results with jakubs */
        int same = 1;
        for(int j = 0; j < number_of_cities; j++) {
            int my_connection = 0;
            for(int k = 0; k < number_of_cities; k++) {
                my_connection += temp[k] * connections[j][k] - random[k] * jakubs[j][k];
            }

            printf("%i\n", my_connection);
            if(my_connection != 0) {
                same = 0;
                break;                
            }
        }

        same ? printf("TAK\n") : printf("NIE\n");

    }

    return 0;
}