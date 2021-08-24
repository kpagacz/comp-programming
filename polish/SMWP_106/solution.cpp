// link to the problem https://pl.spoj.com/problems/SMWP_106/
#include <iostream>
#include<math.h>

void test_case() {
	int edges;
	std::cin >> edges;
	double something = std::sqrt(8.0 * edges + 1);
	if (std::trunc(something) != something) {
		std::cout << "NIE\n";
		return;
	}
	else {
		if ((int)std::trunc(something) % 2 != 1) {
			std::cout << "NIE\n";
		}
		else {
			std::cout << "TAK\n";
		}
	}
}

int main() {
	std::ios_base::sync_with_stdio(false);
	std::cin.tie(NULL);
	int tests;
	std::cin >> tests;

	while (tests) {
		test_case();
		--tests;
	}
}