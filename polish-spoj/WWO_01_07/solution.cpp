// link to the problem https://pl.spoj.com/problems/WWO_01_07/
#define _CRT_SECURE_NO_WARNINGS
#include<cstdio>

int main() {
	int a, b;
	scanf("%d %d", &a, &b);
	for (int i = a; i <= b; ++i) {
		switch(i % 6) {
			case 0:
				printf("ab");
				break;
			case 2: case 4:
				printf("a");
				break;
			case 3:
				printf("b");
				break;
		}
	}
}