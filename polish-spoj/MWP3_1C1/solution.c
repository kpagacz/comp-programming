#include<stdio.h>
#include<math.h>

long GCD(long a, long b) {
    int tmp;
    while(a % b != 0) {
        tmp = a % b;
        a = b;
        b = tmp;
    }
    return b;
}

int main(void) {
    int tests;
    scanf("%i", &tests);
    long int a, b;
    for(int i = 0; i < tests; i++) {
        scanf("%li %li", &a, &b);

        if(a == b) {
            int prime = 1;
            for(long i = 2; i < floor(sqrt(a)); i++) {
                if(a % i == 0) {
                    printf("%li\n", a / i);
                    prime = 0;
                    break;
                }
            }
            if(prime) {
                printf("1\n");
            }
        } else {
            printf("%li\n", GCD(a, b));
        }
    }
    return 0;
}