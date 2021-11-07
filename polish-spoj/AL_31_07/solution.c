#include<stdio.h>
#include<math.h>
#include<stdlib.h>

int minus_plus_minus_repetitions(int a, int level) {
    int expon = pow(2, level);
    if((a % expon) == 0) {
        return level;
    } else {
        a = a % expon;
        level--;
        return minus_plus_minus_repetitions(a, level);
    }
}

int main(void) {
    int level;
    scanf("%i", &level);

    if(level == 1) {
        printf("+");
        return 0;
    }

    for(int i = 1; i < pow(2, level); i++) {
        if(i % 2 == 1) {
            for(int j = 0; j < pow(2, level - 1) - 1; j++) {
                printf("+|");
            }
            printf("+\n");
        } else {
            int repeats_no = minus_plus_minus_repetitions(i, level - 1);
            int minuses = pow(2, repeats_no) - 1;
            repeats_no = pow(2, level - 1 - repeats_no);

            for(int k = 1; k < repeats_no; k++) {
                for(int j = 0; j < minuses; j++) {
                    printf("-");
                }
                printf("+");
                for(int j = 0; j < minuses; j++) {
                    printf("-");
                }
                printf("|");
            }

            for(int j = 0; j < minuses; j++) {
                printf("-");
            }
            printf("+");
            for(int j = 0; j < minuses; j++) {
                printf("-");
            }
            printf("\n");
        }
    }

    return 0;
}