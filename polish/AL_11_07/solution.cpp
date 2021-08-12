#define _CRT_SECURE_NO_WARNINGS
#include <stdio.h>
#include <string>

int main() {
	char first, second;
	int read_chars;
	while ((read_chars = scanf("%c%c", &first, &second)) && read_chars > 0) {
		if (read_chars == 1) {
			printf("%c", first);
			break;
		}
		if (second == '\n' || first == '\n') {
			printf("%c%c", first, second);
			continue;
		}
		printf("%c%c", second, first);
	}
}