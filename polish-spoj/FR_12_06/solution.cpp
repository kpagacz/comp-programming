// link to the problem https://pl.spoj.com/problems/FR_12_06/
#include <iostream>

int main() {
	char mark;
	int pluses, minuses;
	pluses = minuses = 0;
	while (std::cin.get(mark)) {
		if (mark == '+') ++pluses; else if (mark == '-') ++minuses;
	}

	if (pluses > minuses && (pluses - minuses) / 3 > 0) {
		for (int i = 0; i < -1 + (pluses - minuses) / 3; ++i) std::cout << "5 ";
		std::cout << "5";
	}
	else if (minuses > pluses && (minuses - pluses) / 3 > 0) {
		for (int i = 0; i < -1 + (minuses - pluses) / 3; ++i) std::cout << "1 ";
		std::cout << "1";
	}
	else {
		std::cout << "BRAK";
	}
}