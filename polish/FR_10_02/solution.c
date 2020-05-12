#include<stdio.h>


int main(void) {
    int chars_number;
    scanf("%i", &chars_number);

    char chars[chars_number + 1];
    for(int i = 0; i < chars_number; i++) {
        char c;
        scanf(" %c", &c);
        chars[i] = c;
    }
    chars[chars_number] = '\0';

    char word[1001];
    scanf("%s", &word);

    int to_repeat = 0;
    for(int i = 0; word[i] != '\0'; i++) {
        to_repeat = 0;
        for(int j = 0; chars[j] != '\0'; j++) {
            if(word[i] == chars[j]) {
                to_repeat = 1;
                break;
            }
        }

        if(to_repeat) {
            printf("%c%c", word[i], word[i]);
        } else {
            printf("%c", word[i]);
        }
    }

    return 0;
}