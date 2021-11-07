#include <iostream>

// link to the problem https://pl.spoj.com/problems/FR_AA_05/
int main() {
	int radius, a;
	std::cin >> a >> radius;
	int square_radius = radius * radius;

	int cases;
	std::cin >> cases;
	int missed_insects = 0;
	for (int i = 0; i < cases; i++) {
		int x, y;
		std::cin >> x >> y;
		if ((x - a) * (x - a) + y * y > square_radius) ++missed_insects;
	}

	std::cout << missed_insects;
}