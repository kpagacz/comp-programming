#include<stdio.h>

/* uses lemma:
Let M be an nth power of a connection matrix.
Then Mij is equal to number of routes from i to j
that contain n vertices
*/

/* passes 
could use a faster matrix multiplication and of course IO
There is also a trick to make (M^2 - N) * V calculate in O(n^2)
and use the fact that multiplying a vector of unsigned values 
with zero matrix should give 0 matrix, but it is a pure hell for
me to implement
*/


void multiplyMatrices(int m1[][100], int m2[][100], int res[][100], int dim) {
    /* naive */
    for(int i = 0; i < dim; i++) {
        for(int j = 0; j < dim; j++) {
            int sum = 0;
            for(int k = 0; k < dim; k++) {
                sum += m1[i][k] * m2[k][j];
            }
            res[i][j] = sum;
        }
    }
}

int main(void) {
    int tests;
    scanf(" %i", &tests);
    int connections[100][100];
    int number_of_cities;

    for(int i = 0; i < tests; i++) {
        int my_results[100][100] = {{0}};   
        scanf("%i", &number_of_cities);
        /* read in connections */
        for(int k = 0; k < number_of_cities; k++) {
            for(int l = 0; l < number_of_cities; l++) {
                scanf("%i", &connections[k][l]);
            }
        }

        /* multiply matrix */
        multiplyMatrices(connections, connections, my_results, number_of_cities);

        /* check for equality */
        int same = 1;
        int num;
        for(int k = 0; k < number_of_cities; k++) {
            for(int l = 0; l < number_of_cities; l++) {
                scanf("%i", &num);
                if(num != my_results[k][l]) {
                    same = 0;
                }
            }
        }

        same ? printf("TAK\n") : printf("NIE\n");
    }

    return 0;
}