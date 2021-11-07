#include<stdio.h>

int main(void) {
    int tests;
    scanf("%i", &tests);
    getchar();

    char word[10001];
    int min = 32, max = 0;
    
    for(int i = 0; i < tests; i++) {
        min = 32;
        max = 0;
        fgets(word, 10001, stdin);
        for(int j = 0; word[j] != '\n'; j++) {
            if(word[j] - 'A' < min) {
                min = word[j] - 'A';
            }

            if(word[j] - 'A' > max) {
                max = word[j] - 'A';
            }
        }
        
        printf("%i\n", max - min);

    }

    return 0;
}