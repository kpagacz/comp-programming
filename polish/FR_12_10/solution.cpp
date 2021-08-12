// link to the problem https://pl.spoj.com/problems/FR_12_10/
#include <iostream>

void test_case(int width, int height) {
	for (int i = 0; i < height - 1; ++i) {
		for (int j = 0; j < width; ++j) {
			if ((i + j) % 2 == 0) std::cout << "W"; else std::cout << "B";
		}
		std::cout << "\n";
	}

	for (int i = 0; i < width - 1; ++i) {
		if ((i + height - 1) % 2 == 0) std::cout << "W"; else std::cout << "B";
	}

	if (width % 2 == 0 && height % 2 == 0) std::cout << "B"; else std::cout << "W";
	std::cout << "\n";
}

int main() {
	int test_cases;
	std::cin >> test_cases;

	int width, height;
	for (int i = 0; i < test_cases; i++) {
		std::cin >> width >> height;
		test_case(width, height);
	}
}