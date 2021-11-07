#include <stdio.h>

int main() {
    int loops;
    scanf("%i", &loops);
    char first[1001];
    char second[1001];


    for(int i = 0; i < loops; i++) {
        scanf("%s", &first);
        scanf("%s", &second);

        int possible = 0;
        int second_index = 0;
        for(int j = 0; first[j] != '\0'; j++) {
            if(first[j] == second[second_index]) {
                    second_index++;
                }
            if(second[second_index] == '\0') {
                possible = !possible;
                printf("Tak\n");
                break;
            }
        }
        if(!possible) {
            printf("Nie\n");
        }
    }

    return 0;
}