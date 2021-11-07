// link to the problem https://pl.spoj.com/problems/FR_08_09/
#define _CRT_SECURE_NO_WARNINGS
#include<stdio.h>
#include<cstdint>
#include<cstring>

constexpr int64_t gcf(int64_t a, int64_t b) {
	if ((b - a) % a == 0) {
		return a;
	}
	else {
		int temp = (b - a) % a;
		b = a;
		a = temp;
		return gcf(a, b);
	}
}


int main() {
	int tests;
	scanf("%d", &tests);
	for (int i = 0; i < tests; ++i) {
		uint64_t a, n, d;
		scanf(" %lld %lld %lld", &a, &n, &d);
		int64_t common_factor = gcf(n, d);
		n /= common_factor;
		d /= common_factor;

		if (a == 0) {
			printf("%lld %lld\n", d, n);
		}
		else {
			n += d * a;
			common_factor = gcf(n, d);
			n /= common_factor;
			d /= common_factor;
			printf("%lld %lld\n", d, n);
		}
	}
}