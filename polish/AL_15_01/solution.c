#include <stdio.h>
#include <stdlib.h>
/* surprisingly for me gets WA, despite passing my additional test */

int main(void) {
    long long int length = 1000001;

    char* darek = (char*)malloc(length * sizeof(char));
    char* jarek = (char*)malloc(length * sizeof(char));
    char* marek = (char*)malloc(length * sizeof(char));

    fgets(darek, length, stdin);
    fgets(jarek, length, stdin);
    fgets(marek, length, stdin);

    long long int index = 0;
    long long int maximum = 0;

    while(darek[index] != '\0') {
        int a = darek[index] != jarek[index];
        int b = darek[index] != marek[index];
        int c = marek[index] == jarek[index];
        maximum += a*c + b*c + a*!b*!c + !a*b*!c + !c*a*b;
        index++;
    }


    free(darek);
    free(jarek);
    free(marek);

    printf("%li", maximum);
    return 0;
}