#include<stdio.h>

/* to rozwiązanie dostaje TLE */

void findConnections(int origin, int target, int current, int connections[][100], int number_of_cities, int my_results[][100], int hops) {
    if(hops >= 0) {
        for(int i = 0; i < number_of_cities; i++) {
            if(connections[current][i] == 1) {
                    findConnections(origin, target, i, connections, number_of_cities, my_results, hops - 1);
            }
        }
    } else {
        if(current == target) {
            my_results[origin][target]++;
        }                
    }
}

int areMatricesEqual(int matrix1[][100], int matrix2[][100], int dim) {
    int resp = 1;

    for(int i = 0; i < dim; i++) {
        for(int j = 0; j < dim; j++) {
            if(matrix1[i][j] != matrix2[i][j]) {
                return 0;
            }
        }
    }

    return resp;
}

int main(void) {
    int tests;
    scanf(" %i", &tests);
    int connections[100][100];
    int jakub_table[100][100];
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

        /* read in jakub's matrix */
        for(int k = 0; k < number_of_cities; k++) {
            for(int l = 0; l < number_of_cities; l++) {
                scanf("%i", &jakub_table[k][l]);
            }
        }

        /* calculate my matrix */
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                findConnections(j, k, j, connections, number_of_cities, my_results, 1);
            }
        }

        /* print jakubs and my results */
        /*
        printf("Jakub:\n");
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                printf("%i ", jakub_table[j][k]);
            }
            printf("\n");
        }
        
        printf("Mine:\n");
        for(int j = 0; j < number_of_cities; j++) {
            for(int k = 0; k < number_of_cities; k++) {
                printf("%i ", my_results[j][k]);
            }
            printf("\n");
        }
        */

        /* compare two matrices */
        int same = areMatricesEqual(jakub_table, my_results, number_of_cities);
        if(same) {
            printf("TAK\n");
        } else {
            printf("NIE\n");
        }

    }


    return 0;
}