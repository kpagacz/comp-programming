#include<stdio.h>

int main(void) {
    int length;
    scanf("%i", &length);

    char word[length];

    scanf("%s", &word);

    for(int i = 0; i <= length / 2; i++) {
        for(int j = 0; j < length / 2 - i; j++) {
            printf(".");
        }

        for(int j = length / 2 - i; j <= length / 2 + i; j++) {
            printf("%c", word[j]);
        }

        for(int j = 0; j < length / 2 - i; j++) {
            printf(".");
        }
        printf("\n");
    }

    return 0;
}