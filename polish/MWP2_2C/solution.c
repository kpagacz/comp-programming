#include<stdio.h>
#include<math.h>

double max(double a, double b) {
    return a > b ? a : b;
}

double min(double a, double b) {
    return a < b ? a : b;
}

int main(void){
    int tests;
    scanf("%i", &tests);
    
    int first_x, first_y, second_x, second_y;
    long first_r, second_r;

    double distance, overlap;

    for(int i = 0; i < tests; i++) {
        scanf("%i %i %li %i %i %li", &first_x, &first_y, &first_r, &second_x, &second_y, &second_r);
        distance = sqrt(pow(first_x - second_x, 2) + pow(first_y - second_y, 2));
        double min_r = min(first_r, second_r);
        /*
        printf("%.2f %.2f %.2f\n", 
            min(max(first_r - distance, 0), second_r), 
            min(max(second_r - distance, 0), first_r), 
            min(min(distance, max(0, first_r + second_r - distance)), min_r));
            */
        overlap = min(max(first_r - distance, 0), second_r) + min(max(second_r - distance, 0), first_r) +
                    min(min(distance, max(0, first_r + second_r - distance)), min_r);
        double rounded = roundf(100 * overlap) / 100;
        printf("%.2f\n", rounded);
    }

    return 0;
}