// link to the problem https://pl.spoj.com/problems/FR_07_05/
#define _CRT_SECURE_NO_WARNINGS
#include<cstdio>
#include<math.h>
#include<cstdio>
#include<algorithm>

struct Pixel {
	int r, g, b;
	Pixel(int r_, int g_, int b_) : r(r_), g(g_), b(b_) {}
	Pixel() = default;
	friend int operator-(Pixel lhs, const Pixel& rhs) {
		return abs(lhs.r - rhs.r) + abs(lhs.g - rhs.g) + abs(lhs.b - rhs.b);
	}
	void print() {
		printf("%d %d %d\n", r, g, b);
	}
};

int main() {
	int x, y;
	scanf("%d %d", &x, &y);

	Pixel* original = new Pixel[x * y];
	int r, g, b;
	for (int i = 0; i < x * y; ++i) {
		scanf(" #%2x%2x%2x", &r, &g, &b);
		original[i] = Pixel(r, g, b);
	}

	int copies;
	scanf("%d", &copies);
	for (int i = 0; i < copies; ++i) {
		int similar_counts = 0;
		for (int j = 0; j < x * y; ++j) {
			scanf(" #%2x%2x%2x", &r, &g, &b);
			if (Pixel(r, g, b) - original[j] <= 5) ++similar_counts;
		}
		printf("%.2f\n", 1.0 * similar_counts / x / y);
	}

	delete[] original;
}