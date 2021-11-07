// link to the problem https://pl.spoj.com/problems/FR_04_02/
#define _CRT_SECURE_NO_WARNINGS
#include<cstdio>

void test_case() {
	int a, b;
	scanf(" %d %d", &a, &b);
	if (a % 2) printf("BRAK"); else printf("%d", (b + a / 2) % a ? (b + a / 2) % a : a);
	printf("\n");
}

int main() {
	int tests;
	scanf("%d", &tests);
	for (int i = 0; i < tests; ++i) test_case();
}