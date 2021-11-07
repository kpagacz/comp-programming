#include<stdio.h>

int main(void) {
    int tests;
    scanf("%i", &tests);

    int elements_no;
    long int elements[10000];
    long element;
    long long sufix_sum, prefix_sum;


    for(int i = 0; i < tests; i++) {
        scanf("%i", &elements_no);
        sufix_sum = 0;
        prefix_sum = 0;

        for(int j = 0; j < elements_no; j++) {
            scanf("%li", &elements[j]);
            sufix_sum += elements[j];
        }



        int prefix_equal_sufix = 0;
        for(int j = 0; j < elements_no - 1; j++) {
            prefix_sum += elements[j];
            sufix_sum -= elements[j];
            if(sufix_sum == prefix_sum) {
                prefix_equal_sufix = 1;
                printf("%i\n", j + 1);
                break;
            }
        }

        if(!prefix_equal_sufix) {
            printf("0\n");
        }


    }

    return 0;
}