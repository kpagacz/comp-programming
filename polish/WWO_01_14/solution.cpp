// link to the problem https://pl.spoj.com/problems/WWO_01_14/
#define _CRT_SECURE_NO_WARNINGS
#include<math.h>
#include<cstdio>


int main() {
	int v1, v2, n;
	scanf("%d %d %d", &n, &v1, &v2);
	printf("%.2f", 1.0 * v2 / v1 * n);
}