// link to the problem https://pl.spoj.com/problems/FR_AA_08/
#include <iostream>

int main() {
	char x1, y1, x2, y2;
	std::cin.get(x1);
	std::cin.get(y1);
	std::cin.get();
	std::cin.get(x2);
	std::cin.get(y2);

	if (abs(x1 - x2) + abs(y1 - y2) == 3 && x1 != x2 && y1 != y2) std::cout << "TAK"; else std::cout << "NIE";
}