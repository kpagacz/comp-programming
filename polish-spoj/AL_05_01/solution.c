#include <stdio.h>
#include <stdlib.h>

int main() {
    unsigned long long number;
    char binary[65];
    int binary_index;
    int last_one_index;

    while(scanf("%llu", &number) == 1) {
        binary_index = 0;

        while(number > 0) {
            if(number % 2 == 1) {
                binary[binary_index] = '1';
                last_one_index = binary_index;
            } else {
                binary[binary_index] = '0';
            }
            binary_index++;
            number = number >> 1;
        }
        binary[binary_index] = '\0';
        printf("%llu\n", strtoull(binary, NULL, 2));
    }
    
    return 0;
}